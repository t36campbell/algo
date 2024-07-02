/// ## Merge Sort
///  
/// It recursively divides an array into smaller halves until each half contains only one element,
/// then merges those halves together in sorted order.
///
/// Time Complexity:\
///     - Best, Worst, Avg: *O(nlogn)\
///
/// ### Example
/// ```rust
/// use cs_prep::sort;
///
/// let arr = vec![1, 3, 5, 7, 2, 4, 6, 8];
/// let sorted = merge::sort(arr);
///
/// assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8]);
/// ```
pub fn sort<T>(arr: Vec<T>) -> Vec<T>
where
    T: Clone + Copy + PartialOrd + std::fmt::Debug,
{
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = (arr.len() / 2) as f32;
    let mid = pivot.floor() as usize;
    let (l, r) = arr.split_at(mid);
    let left = sort(l.to_vec());
    let right = sort(r.to_vec());

    return join(left, right);
}

fn join<T>(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T>
where
    T: Copy + PartialOrd + std::fmt::Debug,
{
    let mut merged = Vec::<T>::new();

    while !left.is_empty() && !right.is_empty() {
        let a = left.first();
        let b = right.first();

        let val = if a <= b {
            left.remove(0)
        } else {
            right.remove(0)
        };

        merged.push(val);
    }

    merged.append(&mut left);
    merged.append(&mut right);
    return merged;
}

#[cfg(test)]
mod tests {
    use crate::merge;

    #[test]
    fn e1() {
        let arr = vec![3, 6, 7, 1, 5, 9, 2, 8, 0, 0, 5, 8];
        let merged = merge::sort(arr);

        assert_eq!(merged, vec![0, 0, 1, 2, 3, 5, 5, 6, 7, 8, 8, 9]);
    }

    #[test]
    fn e2() {
        let arr = vec![
            "executor",
            "law",
            "religion",
            "eyeball",
            "velocity",
            "quartz",
            "convection",
            "wink",
            "distortion",
            "ecclesia",
            "inhibition",
            "secrecy",
        ];
        let merged = merge::sort(arr);

        assert_eq!(
            merged,
            vec![
                "convection",
                "distortion",
                "ecclesia",
                "executor",
                "eyeball",
                "inhibition",
                "law",
                "quartz",
                "religion",
                "secrecy",
                "velocity",
                "wink"
            ]
        );
    }
}
