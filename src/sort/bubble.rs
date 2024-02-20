/// ## Bubble Sort
///  
/// It repeatedly steps through the list, compares adjacent elements,
/// and swaps them if they are in the wrong order until the entire list is sorted.
///
/// Time Complexity:\
///     - Best: *O(n)*\
///     - Avg: *O(n^2)*\
///     - Worst: *O(n^2)*\
///
/// ### Example
/// ```rust
/// let arr = vec![1, 3, 5, 7, 2, 4, 6, 8];
/// let merged = sort::bubble(arr);
///
/// assert_eq!(merged, vec![1, 2, 3, 4, 5, 6, 7, 8]);
/// ```
pub fn bubble<T>(mut arr: Vec<T>) -> Vec<T>
where
    T: Clone + Copy + PartialOrd + std::fmt::Debug,
{
    let mut working = true;
    while working {
        working = false;
        let mut clone = arr.clone();
        for (i, a) in clone.iter_mut().enumerate() {
            if arr.len() == i + 1 {
                continue;
            }

            if a > &mut arr[i + 1] {
                arr.swap(i, i + 1);
                working = true
            }
        }
    }

    return arr;
}

#[cfg(test)]
mod tests {
    use crate::sort;

    #[test]
    fn e1() {
        let arr = vec![3, 6, 7, 1, 5, 9, 2, 8, 0, 0, 5, 8];
        let merged = sort::bubble(arr);

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
        let merged = sort::bubble(arr);

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
