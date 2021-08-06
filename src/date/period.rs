use std::ops::Add;


#[derive(Debug)]
pub struct Days(i64);

impl Add for Days {
    type Output = Days;
    fn add(self, rhs: Self) -> Self::Output {
        Days(self.0 + rhs.0)
    }
}

#[derive(Debug)]
pub struct Months(i64);
impl Add for Months {
    type Output = Months;
    fn add(self, rhs: Self) -> Self::Output {
        Months(self.0 + rhs.0)
    }
}

#[derive(Debug)]
pub struct Years(i64);
impl Add for Years {
    type Output = Years;
    fn add(self, rhs: Self) -> Self::Output {
        Years(self.0 + rhs.0)
    }
}
