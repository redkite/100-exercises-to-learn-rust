// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(other: u16) -> SaturatingU16 {
        SaturatingU16 { value: other }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(other: u8) -> SaturatingU16 {
        SaturatingU16 {
            value: other as u16,
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(other: &u16) -> SaturatingU16 {
        SaturatingU16 {
            value: other.clone(),
        }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(other: &u8) -> SaturatingU16 {
        SaturatingU16 {
            value: other.clone() as u16,
        }
    }
}

use std::ops::Add;
impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(rhs),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.clone()),
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == other.clone()
    }
}
