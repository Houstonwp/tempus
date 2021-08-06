use std::ops::{Add, AddAssign};

use num::Unsigned;

#[derive(Debug)]
pub struct Days<T>(T)
where
    T: Unsigned;

impl<T: Unsigned> Add for Days<T> {
    type Output = Days<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Days(self.0 + rhs.0)
    }
}

impl<T: Unsigned + Copy> AddAssign for Days<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Days(self.0 + rhs.0);
    }
}

#[derive(Debug)]
pub struct Months<T>(T)
where
    T: Unsigned;
impl<T: Unsigned> Add for Months<T> {
    type Output = Months<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Months(self.0 + rhs.0)
    }
}

impl<T: Unsigned + Copy> AddAssign for Months<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Months(self.0 + rhs.0);
    }
}

#[derive(Debug)]
pub struct Years<T>(T)
where
    T: Unsigned;
impl<T: Unsigned> Add for Years<T> {
    type Output = Years<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Years(self.0 + rhs.0)
    }
}

impl<T: Unsigned + Copy> AddAssign for Years<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Years(self.0 + rhs.0);
    }
}
