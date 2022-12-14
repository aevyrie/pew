use std::{hash::Hash, ops::Add};

use bevy::reflect::Reflect;

pub trait GridPrecision:
    Default
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Hash
    + Copy
    + Clone
    + Send
    + Sync
    + Reflect
    + Add
    + std::fmt::Debug
    + std::fmt::Display
    + 'static
{
    const ZERO: Self;
    const ONE: Self;
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn as_f64(self) -> f64;
    fn from_f64(input: f64) -> Self;
}

impl GridPrecision for i8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        Self::wrapping_add(self, rhs)
    }
    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self {
        Self::wrapping_sub(self, rhs)
    }
    #[inline]
    fn as_f64(self) -> f64 {
        self as f64
    }
    #[inline]
    fn from_f64(input: f64) -> Self {
        input as Self
    }
}

impl GridPrecision for i16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        Self::wrapping_add(self, rhs)
    }
    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self {
        Self::wrapping_sub(self, rhs)
    }
    #[inline]
    fn as_f64(self) -> f64 {
        self as f64
    }
    #[inline]
    fn from_f64(input: f64) -> Self {
        input as Self
    }
}

impl GridPrecision for i32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        Self::wrapping_add(self, rhs)
    }
    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self {
        Self::wrapping_sub(self, rhs)
    }
    #[inline]
    fn as_f64(self) -> f64 {
        self as f64
    }
    #[inline]
    fn from_f64(input: f64) -> Self {
        input as Self
    }
}

impl GridPrecision for i64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        Self::wrapping_add(self, rhs)
    }
    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self {
        Self::wrapping_sub(self, rhs)
    }
    #[inline]
    fn as_f64(self) -> f64 {
        self as f64
    }
    #[inline]
    fn from_f64(input: f64) -> Self {
        input as Self
    }
}

impl GridPrecision for i128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        Self::wrapping_add(self, rhs)
    }
    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self {
        Self::wrapping_sub(self, rhs)
    }
    #[inline]
    fn as_f64(self) -> f64 {
        self as f64
    }
    #[inline]
    fn from_f64(input: f64) -> Self {
        input as Self
    }
}
