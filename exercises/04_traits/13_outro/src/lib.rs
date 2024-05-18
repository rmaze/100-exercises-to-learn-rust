// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value. OK
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`. OK
//   It should support addition with a right-hand side of type OK
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`. OK
//   It should be possible to compare it with another `SaturatingU16` or a `u16`. OK
//   It should be possible to print its debug representation. OK
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SaturatingU16(u16);

impl SaturatingU16 {
    pub fn new(value: u16) -> Self {
        Self(value)
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self(value as u16)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self(*value)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self(*value as u16)
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: u16) -> Self {
        Self(self.0.saturating_add(other))
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &u16) -> Self {
        Self(self.0.saturating_add(*other))
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &SaturatingU16) -> Self {
        Self(self.0.saturating_add(other.0))
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<SaturatingU16> for u16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        *self == other.0
    }
}
