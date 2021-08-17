use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    fmt,
    ops::{Add, AddAssign, BitAnd, Sub, SubAssign},
};

use num::{Bounded, Integer, NumCast};

use self::period::Months;
pub mod period;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct SerialDate<T: Integer> {
    pub rd: T,
}

impl fmt::Display for SerialDate<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rata die: {}", self.rd)
    }
}

impl SerialDate<u32> {
    pub fn to_field_date(self) -> FieldDate<u32> {
        FieldDate::<u32>::from(self)
    }

    pub fn to_weekday(self) -> Weekday {
        Weekday::from(self)
    }
}

impl Add<period::Days<u32>> for SerialDate<u32> {
    type Output = SerialDate<u32>;
    fn add(self, rhs: period::Days<u32>) -> Self::Output {
        SerialDate {
            rd: self.rd + rhs.0,
        }
    }
}

impl AddAssign<period::Days<u32>> for SerialDate<u32> {
    fn add_assign(&mut self, rhs: period::Days<u32>) {
        *self = self.add(rhs)
    }
}

impl Sub<period::Days<u32>> for SerialDate<u32> {
    type Output = SerialDate<u32>;
    fn sub(self, rhs: period::Days<u32>) -> Self::Output {
        SerialDate {
            rd: self.rd - rhs.0,
        }
    }
}

impl SubAssign<period::Days<u32>> for SerialDate<u32> {
    fn sub_assign(&mut self, rhs: period::Days<u32>) {
        *self = self.sub(rhs)
    }
}

impl From<FieldDate<u32>> for SerialDate<u32> {
    fn from(date: FieldDate<u32>) -> Self {
        let y1 = date.year;
        let m1 = date.month.0;
        let d1 = date.day.0;

        let j = match m1 {
            1 | 2 => 1,
            _ => 0,
        };

        let y0 = y1 - j;
        let m0 = m1 as u32 + 12 * j;
        let d0 = d1 - 1;

        let q1 = y0 / 100;
        let yc = 1461 * y0 / 4 - q1 + q1 / 4;
        let mc = (979 * m0 - 2919) / 32;
        let dc = d0 as u32;

        let r1 = yc + mc + dc;

        SerialDate { rd: r1 }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Day(pub u8);

impl Day {
    pub const MAX: u8 = 31;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Month(pub u8);

#[derive(Debug)]
pub struct MonthOutofRangeError;

impl Error for MonthOutofRangeError {}

impl fmt::Display for MonthOutofRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Months must be between 1 and 12")
    }
}

macro_rules! month_try_from {
    ($($t: ty)*) => ($(
        impl TryFrom<$t> for Month {
            type Error = MonthOutofRangeError;

            fn try_from(value: $t) -> Result<Self, Self::Error> {
                if (1..=12).contains(&value) {
                    Ok(Month(value.try_into().unwrap()))
                } else {
                    Err(MonthOutofRangeError)
                }
            }
        }
    )*)
}

month_try_from!( usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 );

macro_rules! int_from_month {
    ($($t: ty)*) => ($(
        impl From<Month> for $t {
            fn from(value: Month) -> $t {
                value.0 as $t
            }
        }
    )*)
}

int_from_month!( usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 );

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct FieldDate<T: Integer> {
    pub year: T,
    pub month: Month,
    pub day: Day,
}

impl fmt::Display for FieldDate<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month.0, self.day.0)
    }
}

macro_rules! impl_bounded_field_date {
    ($($t: ty)*) => ($(
        impl Bounded for FieldDate<$t> {
            fn min_value() -> Self {
                FieldDate {
                    year: <$t>::MIN,
                    month: Month(1),
                    day: Day(1),
                }
            }

            fn max_value() -> Self {
                FieldDate {
                    year: <$t>::MAX,
                    month: Month(12),
                    day: Day(31),
                }
            }
        }
    )*)
}

impl_bounded_field_date! { usize u32 u64 u128 isize i32 i64 i128 }

impl FieldDate<u32> {
    pub fn to_serial_date(&self) -> SerialDate<u32> {
        SerialDate::<u32>::from(*self)
    }

    pub fn to_weekday(self) -> Weekday {
        self.to_serial_date().to_weekday()
    }

    pub fn wrapping_add(self, rhs: Months<u32>) -> FieldDate<u32> {
        let FieldDate { year, month, day } = self;
        let y_add: u32 = (<u32 as From<Month>>::from(month) + rhs.0 - 1) / 12;
        let m_add: u8 = ((<u32 as From<Month>>::from(month) + rhs.0 - 1) % 12 + 1)
            .try_into()
            .unwrap();
        let eom = last_day_of_month(
            &(year + y_add),
            &Month(<u8 as From<Month>>::from(month) + m_add),
        )
        .0;

        match day.0 > eom {
            true => FieldDate::new(year + y_add, month.0 + m_add + 1, day.0 - eom),
            false => FieldDate::new(year + y_add, month.0 + m_add, day.0),
        }
    }

    pub fn wrapping_sub(self, rhs: Months<u32>) -> FieldDate<u32> {
        let FieldDate { year, month, day } = self;
        let y_add = rhs.0 / 12;
        let m_add: u8 = (rhs.0 % 12).try_into().unwrap();
        let eom = last_day_of_month(&(year - y_add), &Month(month.0 + m_add)).0;

        match day.0 > eom {
            true => FieldDate::new(year + y_add, month.0 + m_add + 1, day.0 - eom),
            false => FieldDate::new(year + y_add, month.0 + m_add, day.0),
        }
    }

    pub fn new(y: u32, m: u8, d: u8) -> FieldDate<u32> {
        let m0 = m.try_into().unwrap();
        let ldm = last_day_of_month(&y, &m0);
        if d > ldm.0 {
            panic!("{:?} has {:?} days, got {}.", m0, ldm, d);
        }
        let d0 = Day(d);

        FieldDate {
            year: y,
            month: m0,
            day: d0,
        }
    }
}

impl Add<period::Days<u32>> for FieldDate<u32> {
    type Output = FieldDate<u32>;
    fn add(self, rhs: period::Days<u32>) -> Self::Output {
        self.to_serial_date().add(rhs).to_field_date()
    }
}

impl AddAssign<period::Days<u32>> for FieldDate<u32> {
    fn add_assign(&mut self, rhs: period::Days<u32>) {
        *self = self.add(rhs)
    }
}

impl Sub<period::Days<u32>> for FieldDate<u32> {
    type Output = FieldDate<u32>;
    fn sub(self, rhs: period::Days<u32>) -> Self::Output {
        self.to_serial_date().sub(rhs).to_field_date()
    }
}

impl SubAssign<period::Days<u32>> for FieldDate<u32> {
    fn sub_assign(&mut self, rhs: period::Days<u32>) {
        *self = self.sub(rhs)
    }
}

impl From<SerialDate<u32>> for FieldDate<u32> {
    fn from(date: SerialDate<u32>) -> Self {
        let n1 = 4 * date.rd + 3;
        let q1 = n1 / 146097;
        let r1 = n1 % 146097 / 4;

        let p32 = 1_u64 << 32;
        let n2 = 4 * r1 + 3;
        let u2 = 2939745_u64 * n2 as u64;
        let q2 = (u2 / p32) as u32;
        let r2 = (u2 % p32) as u32 / 2939745 / 4;

        let p16 = 1 << 16;
        let n3 = 2141 * r2 + 197913;
        let q3 = n3 / p16;
        let r3 = n3 % p16 / 2141;

        let y0 = 100 * q1 + q2;
        let m0 = q3;
        let d0 = r3;

        let j = match r2 {
            0..=305 => 0,
            _ => 1,
        };

        let y1 = y0 + j;
        let m1 = m0 - 12 * j;
        let d1 = d0 + 1;

        FieldDate {
            year: y1,
            month: Month(m1 as u8),
            day: Day(d1 as u8),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl From<SerialDate<u32>> for Weekday {
    fn from(date: SerialDate<u32>) -> Self {
        match (date.rd + 3) % 7 {
            0 => Weekday::Sunday,
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            _ => panic!("Date is out of bounds."),
        }
    }
}

impl From<u32> for Weekday {
    fn from(n: u32) -> Self {
        match n {
            0 => Weekday::Sunday,
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            _ => panic!("Date is out of bounds."),
        }
    }
}

impl Weekday {
    pub const MAX: u32 = 6;
}

impl Sub for Weekday {
    type Output = period::Days<u32>;

    fn sub(self, rhs: Self) -> Self::Output {
        let w0 = self as u32;
        let w1 = rhs as u32;

        let days_u32 = w0.wrapping_sub(w1);

        let days = match days_u32 {
            0..=6 => days_u32,
            _ => days_u32.wrapping_add(7),
        };

        period::Days(days)
    }
}

impl Sub<period::Days<u32>> for Weekday {
    type Output = Weekday;

    fn sub(self, rhs: period::Days<u32>) -> Self::Output {
        let day_u32 = self as u32;

        let days = day_u32.wrapping_sub(rhs.0);
        let weekday = match days {
            0..=6 => days,
            _ => days.wrapping_add(7),
        };

        Weekday::from(weekday)
    }
}

impl Add<period::Days<u32>> for Weekday {
    type Output = Weekday;
    fn add(self, rhs: period::Days<u32>) -> Self::Output {
        Weekday::from((self as u32 + rhs.0) % 7)
    }
}

pub trait Year: Integer + BitAnd<Output = Self> + NumCast + Copy {}

impl Year for u32 {}
impl Year for i32 {}
impl Year for u64 {}
impl Year for i64 {}
impl Year for isize {}
impl Year for usize {}

pub fn is_leap_year<T>(y: &T) -> bool
where
    T: Year,
{
    let bitop = if *y % NumCast::from(100).unwrap() == T::zero() {
        NumCast::from(15).unwrap()
    } else {
        NumCast::from(3).unwrap()
    };
    *y & bitop == NumCast::from(0).unwrap()
}

pub fn last_day_of_month<T>(y: &T, m: &Month) -> Day
where
    T: Year,
{
    match m.0 {
        2 if is_leap_year(y) => Day(29),
        2 => Day(28),
        _ => Day((m.0 ^ (m.0 >> 3)) | 30),
    }
}

pub trait Calendar<Y: Year, R: Year> {
    const EPOCH: FieldDate<Y>;

    fn to_serial_date(u1: &FieldDate<Y>) -> SerialDate<R>;
    fn to_field_date(r0: SerialDate<R>) -> FieldDate<Y>;
}

pub struct UGregorian {}

impl Calendar<u32, u32> for UGregorian {
    const EPOCH: FieldDate<u32> = FieldDate {
        year: 0,
        month: Month(3),
        day: Day(1),
    };

    fn to_serial_date(u1: &FieldDate<u32>) -> SerialDate<u32> {
        u1.to_serial_date()
    }

    fn to_field_date(r0: SerialDate<u32>) -> FieldDate<u32> {
        r0.to_field_date()
    }
}

// pub const U_GREGORIAN: Calendar<u32> = Calendar {
//     epoch: FieldDate {
//         year: 0,
//         month: Month(3),
//         day: Day(1),
//     },
// };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_date_new() {
        assert_eq!(
            FieldDate::new(2000, 1, 1),
            FieldDate {
                year: 2000,
                month: Month(1),
                day: Day(1)
            }
        );
    }

    #[test]
    #[should_panic]
    fn field_date_new_invalid() {
        FieldDate::new(2000, 14, 1);
    }

    #[test]
    fn field_date_roundtrip() {
        let unix_epoch = FieldDate::new(1970, 1, 1);
        assert_eq!(unix_epoch, unix_epoch.to_serial_date().to_field_date());
    }

    #[test]
    fn serial_date_roundtrip() {
        let serial_date = SerialDate { rd: 10000 };
        assert_eq!(serial_date, serial_date.to_field_date().to_serial_date());
    }

    #[test]
    fn unix_epoch() {
        let unix_epoch = FieldDate::new(1970, 1, 1).to_serial_date();
        let unix_weekday = unix_epoch.to_weekday();
        assert_eq!(unix_weekday, Weekday::Thursday);
        assert_eq!(unix_weekday - period::Days(1), Weekday::Wednesday);
        assert_eq!(unix_weekday + period::Days(1), Weekday::Friday);
    }

    #[test]
    fn weekday_add() {
        assert_eq!(Weekday::Monday + period::Days(2), Weekday::Wednesday);
        assert_eq!(Weekday::Wednesday + period::Days(7), Weekday::Wednesday);
        assert_eq!(Weekday::Saturday + period::Days(2), Weekday::Monday);
    }

    #[test]
    fn weekday_sub_period() {
        assert_eq!(Weekday::Monday - period::Days(2), Weekday::Saturday);
        assert_eq!(Weekday::Wednesday - period::Days(7), Weekday::Wednesday);
        assert_eq!(Weekday::Saturday - period::Days(2), Weekday::Thursday);
    }

    #[test]
    fn weekday_sub_weekday() {
        assert_eq!(Weekday::Monday - Weekday::Saturday, period::Days(2));
        assert_eq!(Weekday::Wednesday - Weekday::Wednesday, period::Days(0));
        assert_eq!(Weekday::Saturday - Weekday::Thursday, period::Days(2));
    }
}
