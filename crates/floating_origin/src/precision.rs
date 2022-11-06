use std::ops::Add;

use bevy::reflect::Reflect;

pub trait Precision:
    Default + Eq + PartialEq + Copy + Clone + Send + Sync + Reflect + Add + std::fmt::Debug + std::fmt::Display+ 'static
{
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn one() -> Self;
    fn as_f64(self) -> f64;
}

impl Precision for i128 {
    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        Self::wrapping_add(self, rhs)
    }
    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self  {
        Self::wrapping_sub(self, rhs)
    }
    #[inline]
    fn one() -> Self {
        1
    }
    #[inline]
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl Precision for i64 {
    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        Self::wrapping_add(self, rhs)
    }
    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self  {
        Self::wrapping_sub(self, rhs)
    }
    #[inline]
    fn one() -> Self {
        1
    }
    #[inline]
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl Precision for i32 {
    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        Self::wrapping_add(self, rhs)
    }
    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self  {
        Self::wrapping_sub(self, rhs)
    }
    #[inline]
    fn one() -> Self {
        1
    }
    #[inline]
    fn as_f64(self) -> f64 {
        self as f64
    }
}
