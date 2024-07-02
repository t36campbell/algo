/// Solution for LeetCode 33. Search in Rotated Sorted Array
fn set_n(arr: &[i32], target: i32, mid: usize) -> i32 {
    if (arr[mid] < arr[0]) == (target < arr[0]) {
        return arr[mid];
    }

    if target < arr[0] {
        return i32::MIN;
    }

    return i32::MAX;
}

pub fn search(arr: Vec<i32>, target: i32) -> i32 {
    let (mut lo, mut hi) = (0, arr.len());

    while lo < hi {
        let mid = (lo + hi) / 2;
        let num = set_n(&arr, target, mid);

        match num {
            _ if num < target => lo = mid + 1,
            _ if num > target => hi = mid,
            _ => return mid as i32,
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn e1() {
        let arr = vec![4, 5, 6, 7, 0, 1, 2];
        let found = search(arr, 1);
        assert_eq!(found, 5)
    }

    #[test]
    fn e2() {
        let arr = vec![4, 5, 6, 7, 0, 1, 2];
        let found = search(arr, 3);
        assert_eq!(found, -1)
    }

    #[test]
    fn e3() {
        let arr = vec![4, 5, 6, 7, 0, 1, 2];
        let found = search(arr, 7);
        assert_eq!(found, 3)
    }

    #[test]
    fn e4() {
        let arr = vec![4, 5, 6, 7, 0, 1, 2];
        let found = search(arr, 8);
        assert_eq!(found, -1)
    }
}
