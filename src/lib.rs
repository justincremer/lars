use std::ops::{Add, Div, Mul, Sub};

/* TODO:
FLOAT
sqrt(&self) -> Vec<T>
pow(&self, other: Vec<T>) -> Vec<T>
sin(&self) -> Vec<T>
cos(&self) -> Vec<T>
min(&self, other: Vec<T>) -> Vec<T>
max(&self, other: Vec<T>) -> Vec<T>
lerp(&self, others: (Vec<T>, Vec<T>)) -> Vec<T>

INTEGER
sqrlen(&self) -> i32
 */

// Interpolate a set of primitives
pub fn lerp<T>(n: [T; 3]) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    n[0] + (n[1] - n[0]) * n[2]
}

pub trait Interporable<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn from_single(n: T, s: usize) -> Self;
    fn sum(&self, other: Vec<T>) -> Self;
    fn sub(&self, other: Vec<T>) -> Self;
    fn mul(&self, other: Vec<T>) -> Self;
    fn div(&self, other: Vec<T>) -> Self;
}

impl<T> Interporable<T> for Vec<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn from_single(n: T, s: usize) -> Self {
        vec![n; s]
    }

    fn sum(&self, other: Vec<T>) -> Self {
        self.iter()
            .enumerate()
            .map(|(i, n)| *n + other[i])
            .collect()
    }

    fn sub(&self, other: Vec<T>) -> Self {
        self.iter()
            .enumerate()
            .map(|(i, n)| *n - other[i])
            .collect()
    }

    fn mul(&self, other: Vec<T>) -> Self {
        self.iter()
            .enumerate()
            .map(|(i, n)| *n * other[i])
            .collect()
    }

    fn div(&self, other: Vec<T>) -> Self {
        self.iter()
            .enumerate()
            .map(|(i, n)| *n / other[i])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_works() {
        let data = [3.0, 4.0, 5.0];
        assert_eq!(8.0, lerp(data));
    }
}
