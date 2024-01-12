mod primitive;
mod set;

use primitive::{Natural, Odd};
use set::Set;

pub fn synthesize_multiplier_block(t: &Set<Natural>) {
    let mut t = preprocess(t);
    let b = max_bitwidth(&t);
}

fn preprocess(t: &Set<Natural>) -> Set<Odd> {
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
        if l1 == 0 || l2 == 0 {
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

// fn vertex_fundamental_set(u: &HashSet<u64>, v: &HashSet<u64>) -> HashSet<u64> {}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive::*;
    use set::*;

    #[test]
    fn preprocess_test() {
        let t = Set::<Natural>::from_u64([1, 2, 4, 6, 7]);
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
