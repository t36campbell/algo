/// 1. **Creation/Initialization**:
///    - `new()`: Creates a new, empty vector.
///    - `with_capacity(capacity: usize)`: Creates a new vector with the specified capacity.

/// 2. **Adding/Removing Elements**:
///    - `push(item: T)`: Appends an element to the end of the vector.
///    - `pop() -> Option<T>`: Removes and returns the last element from the vector.
///    - `insert(index: usize, element: T)`: Inserts an element at the specified index.
///    - `remove(index: usize) -> T`: Removes and returns the element at the specified index.

/// 3. **Accessing Elements**:
///    - `get(index: usize) -> Option<&T>`: Returns a reference to the element at the specified index, or `None` if the index is out of bounds.
///    - `get_mut(index: usize) -> Option<&mut T>`: Returns a mutable reference to the element at the specified index, or `None` if the index is out of bounds.
///    - `first() -> Option<&T>`: Returns a reference to the first element, or `None` if the vector is empty.
///    - `last() -> Option<&T>`: Returns a reference to the last element, or `None` if the vector is empty.

/// 4. **Size/Capacity**:
///    - `len() -> usize`: Returns the number of elements in the vector.
///    - `is_empty() -> bool`: Returns `true` if the vector is empty, `false` otherwise.
///    - `capacity() -> usize`: Returns the total capacity of the vector.

/// 5. **Iteration**:
///    - `iter() -> Iter<'_, T>`: Returns an iterator over the elements of the vector.
///    - `iter_mut() -> IterMut<'_, T>`: Returns a mutable iterator over the elements of the vector.

/// 6. **Searching/Sorting**:
///    - `contains(&item: &T) -> bool`: Returns `true` if the vector contains the specified item, `false` otherwise.
///    - `sort()`: Sorts the elements of the vector in ascending order.

/// 7. **Conversion**:
///    - `as_slice() -> &[T]`: Converts the vector into a slice.
///    - `as_mut_slice() -> &mut [T]`: Converts the vector into a mutable slice.

/// 8. **Concatenation**:
///    - `append(&mut other: Vec<T>)`: Appends all elements from another vector to the end of this vector.

pub trait Array<T> {
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn get(&self, index: usize) -> Option<&T>;
    fn set(&mut self, value: T, index: usize);
}

pub struct DynamicArray<'a, T> {
    array: &'a mut [Option<T>],
    len: usize,
}

#[cfg(not(tarpaulin_include))]
impl<'a, T> Default for DynamicArray<'a, T>
where
    T: Copy + Default,
{
    fn default() -> DynamicArray<'a, T> {
        todo!()
    }
}

#[cfg(not(tarpaulin_include))]
impl<'a, T> Array<T> for DynamicArray<'a, T> {
    /// where the last item in memory is - not the actual length of the array
    fn len(&self) -> usize {
        todo!()
    }

    /// the actual size fo the array
    fn capacity(&self) -> usize {
        todo!()
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index > self.len {
            todo!("raise index out of bounds")
        }
        match self.array.get(index) {
            Some(v) => v.as_ref(),
            None => None,
        }
    }

    fn set(&mut self, value: T, index: usize) {
        self.array[index] = Some(value)
    }
}

#[cfg(not(tarpaulin_include))]
impl<'a, T> DynamicArray<'a, T>
where
    T: Copy + Default,
{
    fn init(array: Option<&'a mut [Option<T>]>) -> Self {
        let array: &mut [Option<T>] = match array {
            Some(a) => a,
            None => Self::default().array,
        };
        Self { array, len: 0 }
    }

    fn new() -> Self {
        Self::init(None)
    }

    fn size(&self) -> usize {
        return self.array.len();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn dynamic_array() {}
}
