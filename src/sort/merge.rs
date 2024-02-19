fn join<T>(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T>
where
    T: Copy + PartialOrd + std::fmt::Debug,
{
    let mut merged = Vec::<T>::new();

    while !left.is_empty() && !right.is_empty() {
        let a = left.first();
        let b = right.first();

        let val = if a <= b { a.unwrap() } else { b.unwrap() };
        merged.push(*val);
        if a <= b {
            left.remove(0)
        } else {
            right.remove(0)
        };
    }

    merged.append(&mut left);
    merged.append(&mut right);
    return merged;
}

/// Implementation of Merge Sort.
///  
/// It recursively divides an array into smaller halves until each half contains only one element,
/// then merges those halves together in sorted order.
pub fn merge<T>(arr: Vec<T>) -> Vec<T>
where
    T: Clone + Copy + PartialOrd + std::fmt::Debug,
{
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = (arr.len() / 2) as f32;
    let mid = pivot.floor() as usize;
    let (l, r) = arr.split_at(mid);
    let left = merge(l.to_vec());
    let right = merge(r.to_vec());

    return join(left, right);
}
