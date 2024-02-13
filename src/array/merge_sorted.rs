pub fn merge<'a>(mut nums1: &'a mut Vec<i32>, m: i32, nums2: &'a mut Vec<i32>, n: i32) {
    // assumption m and n are always accurate
    if n <= 0 {
        return;
    }
    
    if m <= 0 {
        nums1 = nums2;
        return;
    }

    let max = if m > n { m - 1 } else { n - 1 } as usize;
    let mut index = 0;
    let mut inserts = 0;
    while index <= max {
        let zero = 0_i32;
        let offset = index + inserts;
        let a = nums1.get(offset).unwrap_or(&zero);
        let b = nums2.get(index).unwrap_or(&zero);

        if a == &zero && b != &zero {
            nums1.push(*b);
            index += 1;
            continue;
        }

        if b > a {
            nums1.insert(offset + 1,  *b);
            inserts += 1;
            index += 1;
            continue;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn e1() {
        let mut one = vec![1];
        let mut two = vec![];
        merge(&mut one, 1, &mut two, 0);
        
        print!("\n{:?}\n", one);
        assert_eq!(one, vec![1]);
        assert_eq!(one.len(), 1);
    }
    
    #[test]
    fn e2() {
        let mut one = vec![0];
        let mut two = vec![1];
        merge(&mut one, 0, &mut two, 1);
        
        print!("\n{:?}\n", one);
        assert_eq!(one, vec![1]);
        assert_eq!(one.len(), 1);
    }
    
    #[test]
    fn e3() {
        let mut one = vec![1, 2, 3, 4];
        let mut two = vec![5, 6, 7, 8];
        merge(&mut one, 4, &mut two, 4);
        
        print!("\n{:?}\n", one);
        assert_eq!(one.len(), 8);
        assert_eq!(one, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
    
    #[test]
    fn e4() {
        let mut one = vec![1, 2, 3, 0, 0, 0];
        let mut two = vec![2, 5, 6];
        merge(&mut one, 3, &mut two, 3);

        print!("\n{:?}\n", one);
        assert_eq!(one.len(), 6);
        assert_eq!(one, vec![1, 2, 2, 5, 6]);
    }
}
