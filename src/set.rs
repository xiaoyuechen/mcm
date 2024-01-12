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

use crate::primitive::Nat;
use crate::primitive::Odd;

pub type Set<T> = std::collections::HashSet<T>;

pub trait FromU64<T> {
    fn from_u64(x: T) -> Self;
}

macro_rules! from_x_impl {
    ($t:ty) => {
        impl<const N: usize> FromU64<[u64; N]> for Set<$t> {
            fn from_u64(arr: [u64; N]) -> Self {
                <Set<$t> as FromU64<&[u64]>>::from_u64(&arr)
            }
        }

        impl FromU64<&[u64]> for Set<$t> {
            fn from_u64(x: &[u64]) -> Self {
                x.into_iter().filter_map(|&x| <$t>::new(x)).collect()
            }
        }
    };
}

from_x_impl! {Nat}
from_x_impl! {Odd}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u64_natural_set_test() {
        use crate::primitive::Nat;

        assert_eq!(
            Set::<Nat>::from_u64([0, 1, 1, 2, 3]),
            Set::<Nat>::from(
                [Nat::new(1), Nat::new(2), Nat::new(3)].map(|x| x.unwrap())
            )
        );
    }

    #[test]
    fn from_u64_odd_set_test() {
        assert_eq!(
            Set::<Odd>::from_u64([0, 1, 1, 2, 3]),
            Set::<Odd>::from([Odd::new(1), Odd::new(3)].map(|x| x.unwrap()))
        );
    }
}
