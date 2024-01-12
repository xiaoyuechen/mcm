// mcm --- a Multiple Constant Multiplication solver
// Copyright (C) 2024  Xiaoyue Chen
//
// This file is part of mcm.
//
// mcm is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// mcm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with mcm.  If not, see <http://www.gnu.org/licenses/>.

pub type Nat = core::num::NonZeroU64;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Odd(u64);

impl Odd {
    pub fn new(n: u64) -> Option<Self> {
        if n & 1 == 1 {
            Some(Self(n))
        } else {
            None
        }
    }

    pub const fn used_bits(self) -> u32 {
        self.0.ilog2() + 1
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_new_test() {
        assert_eq!(Odd::new(0), None);
        assert_eq!(Odd::new(1), Some(Odd(1)));
        assert_eq!(Odd::new(2), None);
    }

    #[test]
    fn used_bits_test() {
        assert_eq!(Odd::new(1).unwrap().used_bits(), 1);
        assert_eq!(Odd::new(3).unwrap().used_bits(), 2);
        assert_eq!(Odd::new(5).unwrap().used_bits(), 3);
        assert_eq!(Odd::new(7).unwrap().used_bits(), 3);
        assert_eq!(Odd::new(9).unwrap().used_bits(), 4);
    }
}
