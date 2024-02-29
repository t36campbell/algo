/// ## Insertion Sort
///
/// It iterates through the array, removing one element at a time and
/// inserting it into the correct position in the already sorted part of the array.
///
/// Time Complexity:\
///     - Best: *O(n)*\
///     - Avg: *O(n^2)*\
///     - Worst: *O(n^2)*\
///
/// ### Example
/// ```rust
/// use cs_prep::sort;
///
/// let arr = vec![1, 3, 5, 7, 2, 4, 6, 8];
/// let sorted = sort::insertion(arr);
///
/// assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8]);
/// ```
pub fn insertion<T>(arr: Vec<T>) -> Vec<T>
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
        let merged = sort::insertion(arr);

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
        let merged = sort::insertion(arr);

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
