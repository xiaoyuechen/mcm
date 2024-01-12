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

mod primitive;
mod set;

use primitive::{Nat, Odd};
use set::{FromU64, Set};

pub fn synthesize_multiplier_block(t: &Set<Nat>) -> Set<Odd> {
    let mut t = preprocess(t);
    let b = max_bitwidth(&t);
    let mut r = Set::<Odd>::from_u64([1]);
    let mut w = Set::<Odd>::from_u64([1]);
    let mut s = Set::<Odd>::from_u64([1]);

    while !t.is_empty() {
        // Optimal part
        while !w.is_empty() {
            r.extend(&w);
            s.extend(vertex_fundamental_set(&r, &w, b));
            for wi in &w {
                s.remove(wi);
            }
            w.clear();
            for ti in s.intersection(&t).cloned().collect::<Vec<_>>() {
                synthesize(ti, &mut w, &mut t);
            }
        }
        // Heuristic part
        if !t.is_empty() {
            let s = huristic(&r, &s, &t);
            synthesize(s, &mut w, &mut t)
        }
    }

    r
}

fn huristic(r: &Set<Odd>, s: &Set<Odd>, t: &Set<Odd>) -> Odd {
    todo!()
}

fn synthesize(s: Odd, w: &mut Set<Odd>, t: &mut Set<Odd>) {
    w.insert(s);
    t.remove(&s);
}

fn preprocess(t: &Set<Nat>) -> Set<Odd> {
    t.iter()
        .map(|x| x.get())
        .map(|x| x >> x.trailing_zeros())
        .map(|x| Odd::new(x).unwrap())
        .collect()
}

fn max_bitwidth(t: &Set<Odd>) -> u32 {
    t.iter().max().unwrap().used_bits()
}

struct AConfig {
    l1: u32,
    l2: u32,
    s: bool,
}

impl AConfig {
    fn new(l1: u32, l2: u32, s: bool) -> Option<Self> {
        if l1 == 0 || l2 == 0 && l1 != l2 {
            Some(AConfig { l1, l2, s })
        } else {
            None
        }
    }
}

fn a_op(u: Odd, v: Odd, p: &AConfig) -> Odd {
    let AConfig { l1, l2, s } = *p;
    let t = i128::from(v.get()) << l2;
    let t = if s { t } else { -t };
    let a = ((i128::from(u.get()) << l1) + t).abs();
    Odd::new(a as u64).unwrap()
}

fn vertex_fundamental_set(u: &Set<Odd>, v: &Set<Odd>, b: u32) -> Set<Odd> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preprocess_test() {
        let t = Set::<Nat>::from_u64([1, 2, 4, 6, 7]);
        let tp = preprocess(&t);
        assert_eq!(tp, Set::<Odd>::from_u64([1, 3, 7]));
    }

    #[test]
    fn max_bitwidth_test() {
        assert_eq!(max_bitwidth(&Set::<Odd>::from_u64([3, 1, 7])), 3);
        assert_eq!(max_bitwidth(&Set::<Odd>::from_u64([1, 9])), 4);
    }

    #[test]
    fn a_op_test() {
        let p = AConfig::new(1, 0, true).unwrap();
        let u = Odd::new(5).unwrap();
        let v = Odd::new(7).unwrap();
        assert_eq!(a_op(u, v, &p), Odd::new(17).unwrap());
    }

    #[test]
    fn a_op_test_2() {
        let p = AConfig::new(0, 3, false).unwrap();
        let u = Odd::new(1).unwrap();
        let v = Odd::new(5).unwrap();
        assert_eq!(a_op(u, v, &p), Odd::new(39).unwrap());
    }
}
