/// ## Binary Search
///  
/// ?
/// ?
///
/// Time Complexity:\
///     - Best: *?*\
///     - Avg: *?*\
///     - Worst: *?*\
///
/// ### Example
/// ```rust
/// use cs_prep::search;
///
/// let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
/// let found = search::binary(arr, 6);
///
/// assert_eq!(found, 5);
/// ```
pub fn binary<T>(arr: Vec<T>, k: T) -> i32
where
    T: Clone + Copy + PartialOrd + std::fmt::Debug,
{
    let length = arr.len();
    if length == 0 {
        return -1;
    }

    let first = arr[0];
    if k == first {
        return 0;
    }

    let pivot = (length / 2) as f32;
    let mid = pivot.floor() as usize;
    let (l, r) = arr.split_at(mid);

    let middle = arr[mid];
    if k == middle {
        return mid as i32;
    } else if k > middle {
        let right: Vec<T> = r.to_vec();
        let found = binary(right, k);
        return if found > 0 {
            found + (mid as i32)
        } else {
            found
        };
    } else {
        let left = l.to_vec();
        return binary(left, k);
    }
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn e1() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let found = search::binary(arr, 6);
        assert_eq!(found, 5);
    }

    #[test]
    fn e2() {
        // at the end
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let found = search::binary(arr, 8);
        assert_eq!(found, 7);
    }

    #[test]
    fn e3() {
        // at the beginning
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let found = search::binary(arr, 1);
        assert_eq!(found, 0);
    }

    #[test]
    fn e4() {
        // is middle
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let found = search::binary(arr, 4);
        assert_eq!(found, 3);
    }

    #[test]
    fn e5() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let not_found = search::binary(arr, 0);
        assert_eq!(not_found, -1);
    }

    #[test]
    fn e6() {
        let arr = vec![];
        let not_found = search::binary(arr, 0);
        assert_eq!(not_found, -1);
    }

    #[test]
    fn e7() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let not_found = search::binary(arr, 9);
        assert_eq!(not_found, -1);
    }

    #[test]
    fn e8() {
        let arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let found = search::binary(arr, 0);
        assert_eq!(found, 9);
    }
}
