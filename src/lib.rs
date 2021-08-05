use std::ops::{Add, Mul, Sub};

// Defines linear interpolability of a given data primitive in a higher dimension
pub fn lerp<T>(n: [T; 3]) -> T
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    n[0] + (n[1] - n[0]) * n[2]
}
