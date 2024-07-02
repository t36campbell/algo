use crate::merge;

/// Solution for LeetCode 88. Merge Sorted Array
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if m < nums1.len() as i32 {
        let binding = get_binding(nums1);
        let (n1, _) = binding.split_at(m as usize);
        nums1.clear();
        nums1.append(&mut n1.to_vec());
    }

    if n < nums2.len() as i32 {
        let binding = get_binding(nums2);
        let (n2, _) = binding.split_at(n as usize);
        nums2.clear();
        nums2.append(&mut n2.to_vec());
    }

    if n <= 0 {
        return;
    }

    if m <= 0 {
        nums1.clear();
        nums1.append(nums2);
        return;
    }

    nums1.append(nums2);
    let mut merged = merge::sort(nums1.to_vec());
    nums1.clear();
    nums1.append(&mut merged);
}

fn get_binding(arr: &mut Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    let mut binding = Vec::with_capacity(len);
    binding.clone_from(arr);

    binding
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn e1() {
        let mut one = vec![1];
        let mut two = vec![];
        merge(&mut one, 1, &mut two, 0);

        assert_eq!(one, vec![1]);
        assert_eq!(one.len(), 1);
    }

    #[test]
    fn e2() {
        let mut one = vec![0];
        let mut two = vec![1];
        merge(&mut one, 0, &mut two, 1);

        assert_eq!(one, vec![1]);
        assert_eq!(one.len(), 1);
    }

    #[test]
    fn e3() {
        let mut one = vec![2, 0];
        let mut two = vec![1];
        merge(&mut one, 1, &mut two, 1);

        assert_eq!(one, vec![1, 2]);
        assert_eq!(one.len(), 2);
    }

    #[test]
    fn e4() {
        let mut one = vec![1, 2, 3, 4];
        let mut two = vec![5, 6, 7, 8];
        merge(&mut one, 4, &mut two, 4);

        assert_eq!(one.len(), 8);
        assert_eq!(one, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn e5() {
        let mut one = vec![1, 2, 3];
        let mut two = vec![2, 5, 6];
        merge(&mut one, 3, &mut two, 3);

        assert_eq!(one.len(), 6);
        assert_eq!(one, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn e6() {
        let mut one = vec![1, 3, 5, 7];
        let mut two = vec![2, 4, 6, 8];
        merge(&mut one, 4, &mut two, 4);

        assert_eq!(one.len(), 8);
        assert_eq!(one, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn e7() {
        let mut one = vec![1, 2, 3, 0, 0, 0];
        let mut two = vec![2, 5, 6];
        merge(&mut one, 3, &mut two, 3);

        assert_eq!(one.len(), 6);
        assert_eq!(one, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn e8() {
        let mut one = vec![4, 0, 0, 0, 0, 0];
        let mut two = vec![1, 2, 3, 5, 6];
        merge(&mut one, 1, &mut two, 5);

        assert_eq!(one.len(), 6);
        assert_eq!(one, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn e9() {
        let mut one = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
        let mut two = vec![1, 2, 2];
        merge(&mut one, 6, &mut two, 3);

        assert_eq!(one.len(), 9);
        assert_eq!(one, vec![-1, 0, 0, 1, 2, 2, 3, 3, 3]);
    }
}
