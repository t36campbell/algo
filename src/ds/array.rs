pub trait Array<T> {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<&T>;
    fn set(&mut self, value: T, index: usize);
}

pub struct DynamicArray<'a, T> {
    array: &'a mut [Option<T>],
    len: usize,
}

impl<'a,T> Array<T> for DynamicArray<'a,T> {
    // where the last item in memory is - not the actual length of the array
    fn len(&self) -> usize {
        todo!()
    }

    fn get(&self, index: usize) -> Option<&T> {
        match self.array.get(index) {
            Some(v) => v.as_ref(),
            None => None,
        }
    }

    fn set(&mut self, value: T, index: usize) {
        self.array[index] = Some(value)
    }
}

impl<'a,T> DynamicArray<'a,T> where T: Copy + Default {
    fn init(array: Option<&'a mut [Option<T>]>) -> Self {
        let array: &mut [Option<T>] = match array {
            Some(a) => a,
            None => [None; 64].as_mut(),
        };

        DynamicArray { array, len: 0 }
    }

    fn new() -> Self {
        Self::init(None)
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn dynamic_array() {
    }
}