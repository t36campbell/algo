/// An Implementation of a Dynamic Array that is similar to a Vec
pub struct Array<T> {
    array: Box<[Option<T>]>,
    len: usize,
}

#[cfg(not(tarpaulin_include))]
impl<'a, T> Default for Array<T>
where
    T: Copy + Default,
{
    fn default() -> Array<T> {
        let array = Box::new([None; 64]);
        Self::init(array)
    }
}

#[cfg(not(tarpaulin_include))]
impl<'a, T> Array<T>
where
    T: Copy + Default + Sized + 'a,
{
    fn init(array: Box<[Option<T>]>) -> Array<T> {
        Self { array, len: 0 }
    }

    pub fn new() -> Self {
        Self::default()
    }

    fn resize<const N: usize>(src: [Option<T>; N]) -> Box<[Option<T>]> {
        let mut a = src.to_vec();
        a.resize(N * 2, None);

        return a.into_boxed_slice();
    }

    pub fn from<const N: usize>(src: [Option<T>; N]) -> Array<T> {
        let array = Self::resize(src);

        Self { array, len: N }
    }

    pub fn with_capacity(n: usize) -> Array<T> {
        let array = vec![None; n].into_boxed_slice();
        Self::init(array)
    }

    pub fn reserve(&mut self, additional: usize) {
        let reserved = self.capacity() - self.len;
        if reserved > additional {
            return;
        }

        let len = reserved + additional;
        let mut a = self.array.to_vec();
        a.resize(len, None);
        self.array = a.into_boxed_slice();
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn capacity(&self) -> usize {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if index >= self.len {
            return self.set(index, value);
        }
    }

    fn set(&mut self, index: usize, value: T) {
        self.array[index] = Some(value);
        self.len += 1
    }

    pub fn push(&mut self, value: T) {
        self.set(self.len, value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.remove(self.len)
    }

    pub fn append<const N: usize>(&mut self, other: [Option<T>; N]) {
        todo!()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index > self.len {
            return None;
        }

        match self.array.get(index) {
            Some(v) => v.as_ref(),
            None => None,
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index > self.len {
            return None;
        }

        match self.array.get_mut(index) {
            Some(v) => v.as_mut(),
            None => None,
        }
    }

    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }

    pub fn first_mut(&self) -> Option<&mut T> {
        self.get_mut(0)
    }

    pub fn last(&self) -> Option<&T> {
        todo!()
    }

    pub fn last_mut(&self) -> Option<&mut T> {
        todo!()
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        todo!()
    }

    pub fn to_vec(&self) -> Vec<T> {
        todo!()
    }

    pub fn join(&self, char: char) -> String {
        todo!()
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    pub fn iter(&self) {
        todo!()
    }

    pub fn iter_mut(&self) {
        todo!()
    }

    pub fn as_slice(&self) -> &[T] {
        todo!()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        todo!()
    }

    pub fn drain(&mut self) {
        todo!()
    }

    pub fn contains(&self, x: &T) -> bool {
        todo!()
    }

    pub fn sort(&mut self) {
        todo!()
    }

    pub fn sort_by<F>(&mut self, compare: F) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Array;

    #[test]
    fn dynamic_array_new() {
        let arr: Array<i32> = Array::new();

        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), 64)
    }

    #[test]
    fn dynamic_array_from() {
        let src = [None; 2];
        let arr: Array<i32> = Array::from(src);

        assert_eq!(arr.len(), 2);
        assert_eq!(arr.capacity(), 4)
    }

    #[test]
    fn dynamic_array_with_capacity() {
        let arr: Array<i32> = Array::with_capacity(36);

        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), 36)
    }
}
