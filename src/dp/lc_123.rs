/// Solution for LeetCode 123. Best Time to Buy and Sell Stock III
pub fn max_profit(prices: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    #[ignore]
    fn e1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let max = max_profit(prices);
        assert_eq!(max, 5)
    }

    #[test]
    #[ignore]
    fn e2() {
        let prices = vec![7, 6, 4, 3, 1];
        let max = max_profit(prices);
        assert_eq!(max, 0)
    }

    #[test]
    #[ignore]
    fn e3() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let max = max_profit(prices);
        assert_eq!(max, 7)
    }

    #[test]
    #[ignore]
    fn e4() {
        let prices = vec![1, 2, 3, 4, 5];
        let max = max_profit(prices);
        assert_eq!(max, 4)
    }
}
