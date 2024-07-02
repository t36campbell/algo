/// Solution for LeetCode 53. Maximum Subarray
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut cur = 0;

    for p in nums {
        cur = std::cmp::max(cur, 0) + p;
        max = std::cmp::max(max, cur);
    }

    return max;
}

#[cfg(test)]
mod tests {
    use super::max_sub_array;

    #[test]
    fn e1() {
        let prices = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let sum = max_sub_array(prices);
        assert_eq!(sum, 6)
    }

    #[test]
    fn e2() {
        let prices = vec![5, 4, -1, 7, 8];
        let sum = max_sub_array(prices);
        assert_eq!(sum, 23)
    }

    #[test]
    fn e3() {
        let prices = vec![1];
        let sum = max_sub_array(prices);
        assert_eq!(sum, 1)
    }
}
