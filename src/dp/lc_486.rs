/// Solution for LeetCode 486. Predict the Winner
pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return true;
    }

    let len = nums.len();
    if len % 2 == 0 {
        return true;
    }

    let mut memo = vec![0; len];

    for i in (0..len - 1).rev() {
        for j in i..len {
            if i == j {
                memo[i] = nums[i];
                continue;
            };
            memo[j] = (nums[i] - memo[j]).max(nums[j] - memo[j - 1]);
        }
    }

    memo[len - 1] >= 0
}

#[cfg(test)]
mod tests {
    use super::predict_the_winner;

    #[test]
    fn e1() {
        let arr = vec![1, 5, 2];
        let won = predict_the_winner(arr);
        assert!(!won);
    }

    #[test]
    fn e2() {
        let arr = vec![1, 5, 233, 7];
        let won = predict_the_winner(arr);
        assert!(won);
    }

    #[test]
    fn e3() {
        let arr = vec![0];
        let won = predict_the_winner(arr);
        assert!(won);
    }

    #[test]
    fn e4() {
        let arr = vec![2, 4, 55, 6, 8];
        let won = predict_the_winner(arr);
        assert!(!won);
    }
}
