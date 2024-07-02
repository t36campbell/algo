/// Solution for LeetCode 121. Best Time to Buy and Sell Stock
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut cur = 0;

    for i in 1..prices.len() {
        let profit = prices[i] - prices[i - 1];
        cur = std::cmp::max(cur, 0) + profit;
        max = std::cmp::max(max, cur);
    }

    return max;
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn e1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let max = max_profit(prices);
        assert_eq!(max, 5)
    }

    #[test]
    fn e2() {
        let prices = vec![7, 6, 4, 3, 1];
        let max = max_profit(prices);
        assert_eq!(max, 0)
    }

    #[test]
    fn e3() {
        let prices = vec![1, 2, 3, 4, 5];
        let max = max_profit(prices);
        assert_eq!(max, 4)
    }
}
