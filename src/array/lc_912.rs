use crate::merge;

/// Solution for LeetCode 912. Sort an Array
pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    merge::sort(nums)
}

#[cfg(test)]
mod tests {
    use super::sort_array;

    #[test]
    fn e1() {
        let arr = vec![3, 6, 7, 1, 5, 9, 2, 8, 0, 0, 5, 8];
        let sorted = sort_array(arr);

        assert_eq!(sorted, vec![0, 0, 1, 2, 3, 5, 5, 6, 7, 8, 8, 9]);
    }
}
