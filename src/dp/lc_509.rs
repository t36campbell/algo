/// Solution for LeetCode 509. Fibonacci Number
pub fn fib(n: i32) -> i32 {
    let mut f = std::collections::BTreeMap::new();

    f.insert(0, 0);
    f.insert(1, 1);

    for i in 2..(n + 1) {
        let m1 = i - 1;
        let m2 = i - 2;
        let next = f[&m1] + f[&m2];
        f.insert(i, next);
    }

    *f.get(&n).unwrap()
}

#[cfg(test)]
mod tests {
    use super::fib;

    #[test]
    fn e1() {
        let n = fib(3);
        assert_eq!(n, 2)
    }

    #[test]
    fn e2() {
        let n = fib(4);
        assert_eq!(n, 3)
    }

    #[test]
    fn e3() {
        let n = fib(15);
        assert_eq!(n, 610)
    }
}
