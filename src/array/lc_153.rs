/// Solution for LeetCode 153. Find Minimum in Rotated Sorted Array
pub fn find_min(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let (mut lo, mut hi) = (0, n - 1);

    while lo < hi {
        let float = ((lo + hi) / 2) as f32;
        let mid = float.floor() as usize;

        if nums[mid] > nums[hi] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    nums[lo]
}

#[cfg(test)]
mod tests {
    use super::find_min;

    #[test]
    fn test_e1() {
        let arr = vec![7, 9, 10, 8, 6, 4, 2, 1, 3, 5];
        let found = find_min(arr);
        assert_eq!(found, 1)
    }

    #[test]
    fn test_e2() {
        let arr = vec![4, 2, 1, 3, 5, 7, 9, 10, 8, 6];
        let found = find_min(arr);
        assert_eq!(found, 1)
    }

    #[test]
    fn test_e3() {
        let arr = vec![8, 6, 4, 2, 1, 3, 5, 7, 9, 10];
        let found = find_min(arr);
        assert_eq!(found, 1)
    }
}
