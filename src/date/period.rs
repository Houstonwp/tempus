use std::ops::{Add, AddAssign, Mul};


#[derive(Debug)]
pub struct Days(u64);

impl Add for Days {
    type Output = Days;
    fn add(self, rhs: Self) -> Self::Output {
        Days(self.0 + rhs.0)
    }
}

impl AddAssign for Days {
    fn add_assign(&mut self, rhs: Self) {
        Days(self.0 + rhs.0);
    }
}

impl Mul<u64> for Days {
    type Output = Days;
    fn mul(self, rhs: u64) -> Self::Output {
        Days(self.0 * rhs)
    }
}

#[derive(Debug)]
pub struct Months(u64);
impl Add for Months {
    type Output = Months;
    fn add(self, rhs: Self) -> Self::Output {
        Months(self.0 + rhs.0)
    }
}

impl AddAssign for Months {
    fn add_assign(&mut self, rhs: Self) {
        Months(self.0 + rhs.0);
    }
}

impl Mul<u64> for Months {
    type Output = Months;
    fn mul(self, rhs: u64) -> Self::Output {
        Months(self.0 * rhs)
    }
}

#[derive(Debug)]
pub struct Years(u64);
impl Add for Years {
    type Output = Years;
    fn add(self, rhs: Self) -> Self::Output {
        Years(self.0 + rhs.0)
    }
}

impl AddAssign for Years {
    fn add_assign(&mut self, rhs: Self) {
        Years(self.0 + rhs.0);
    }
}

impl Mul<u64> for Years {
    type Output = Years;
    fn mul(self, rhs: u64) -> Self::Output {
        Years(self.0 * rhs)
    }
}