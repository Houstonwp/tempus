mod dates {
    use std::ops::{Add, Sub};

    use num::Integer;

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct SerialDate<T: Integer> {
        rd: T,
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Day(u8);

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Month(u8);

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct FieldDate<T: Integer> {
        year: T,
        month: Month,
        day: Day,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum Weekday {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

    impl SerialDate<u32> {
        pub fn to_field_date(self) -> FieldDate<u32> {
            FieldDate::<u32>::from(self)
        }

        pub fn to_weekday(self) -> Weekday {
            Weekday::from(self)
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

    impl FieldDate<u32> {
        pub fn to_serial_date(self) -> SerialDate<u32> {
            SerialDate::<u32>::from(self)
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

    impl Day {
        pub const HIGH: u32 = 31;
    }

    impl Weekday {
        pub const HIGH: u32 = 6;
    }

    impl Sub for Weekday {
        type Output = u32;

        fn sub(self, rhs: Self) -> Self::Output {
            let w0 = self as u32;
            let w1 = rhs as u32;

            let days = w0.wrapping_sub(w1);

            match days {
                0..=Self::HIGH => days,
                _ => days.wrapping_add(Self::HIGH),
            }
        }
    }

    impl Sub<u32> for Weekday {
        type Output = Weekday;

        fn sub(self, rhs: u32) -> Self::Output {
            let day_u32 = self as u32;

            let days = day_u32.wrapping_sub(rhs);
            let weekday = match days {
                0..=Self::HIGH => days,
                _ => days.wrapping_add(Self::HIGH),
            };

            Weekday::from(weekday)
        }
    }

    impl Add<u32> for Weekday {
        type Output = Weekday;
        fn add(self, rhs: u32) -> Self::Output {
            Weekday::from((self as u32 + rhs) % 7)
        }
    }

    pub const fn is_leap_year(y: i32) -> bool {
        y & match y % 100 {
            0 => 15,
            _ => 3,
        } == 0
    }

    pub const fn last_day_of_month(y: i32, m: Month) -> Day {
        match m.0 {
            2 if is_leap_year(y) => Day(29),
            2 => Day(28),
            _ => Day((m.0 ^ (m.0 >> 3)) | 30),
        }
    }

    pub struct Calendar<T: Integer> {
        pub epoch: FieldDate<T>,
    }

    pub const U_GREGORIAN: Calendar<u32> = Calendar {
        epoch: FieldDate {
            year: 0,
            month: Month(3),
            day: Day(1),
        },
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
