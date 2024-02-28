/// ## Selection Sort
///
/// It repeatedly selects the smallest element from the unsorted portion of the array and
/// places it at the beginning of the sorted portion.
///
/// Time Complexity:\
///     - Best, Worst, Avg: *O(n^2)*\
///
/// ### Example
/// ```rust
/// let arr = vec![1, 3, 5, 7, 2, 4, 6, 8];
/// let merged = sort::selection(arr);
///
/// assert_eq!(merged, vec![1, 2, 3, 4, 5, 6, 7, 8]);
/// ```
pub fn selection<T>(arr: Vec<T>) -> Vec<T>
where
    T: Clone + Copy + PartialOrd + std::fmt::Debug,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::sort;

    #[test]
    fn e1() {
        let arr = vec![3, 6, 7, 1, 5, 9, 2, 8, 0, 0, 5, 8];
        let merged = sort::selection(arr);

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
        let merged = sort::selection(arr);

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
