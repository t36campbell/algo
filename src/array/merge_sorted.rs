pub fn merge<'a>(nums1: &'a mut Vec<i32>, m: i32, nums2: &'a mut Vec<i32>, n: i32) {
    // assumptions:
    // m and n are always accurate
    nums1.retain(|&i|i != 0);
    if n <= 0 {
        return;
    }
    
    if m <= 0 {
        nums1.clear();
        nums1.append(nums2);
        nums1.retain(|&i|i != 0);
        return;
    }

    let zero = 0_i32;
    let mut index = 0;
    let mut inserts = 0;
    let max = if m > n { m } else { n } as usize;
    while index <= max {
        let ga = nums1.last().unwrap();
        let offset = index + inserts;
        let a = nums1.get(offset).unwrap_or(&zero);
        let b = nums2.get(index).unwrap_or(&zero);

        if a == &zero && b == &zero {
            let el = if a == &zero { *a } else { *b };
            nums1.insert(0,  el);
            inserts += 1;
            index += 1;
            continue;
        }

        if b > ga {
            nums1.push(*b);
            index += 1;
            continue;
        };

        let length = nums1.len();
        let insert = if b > a { offset + 1 } else { offset };
        if insert > length {
            nums1.push(*b);
            continue;
        };
    
        nums1.insert(insert,  *b);
        inserts += 1;
        index += 1;
    }

    nums1.retain(|&i|i != 0);
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
        let mut one = vec![2,0];
        let mut two = vec![1];
        merge(&mut one, 1, &mut two, 1);
        
        print!("\n{:?}\n", one);
        assert_eq!(one, vec![1,2]);
        assert_eq!(one.len(), 2);
    }
    
    #[test]
    fn e4() {
        let mut one = vec![1, 2, 3, 4];
        let mut two = vec![5, 6, 7, 8];
        merge(&mut one, 4, &mut two, 4);
        
        print!("\n{:?}\n", one);
        assert_eq!(one.len(), 8);
        assert_eq!(one, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
    
    #[test]
    fn e5() {
        let mut one = vec![1, 2, 3];
        let mut two = vec![2, 5, 6];
        merge(&mut one, 3, &mut two, 3);

        print!("\n{:?}\n", one);
        assert_eq!(one.len(), 6);
        assert_eq!(one, vec![1, 2, 2, 3, 5, 6]);
    }
    
    #[test]
    fn e6() {
        let mut one = vec![1, 3, 5, 7];
        let mut two = vec![2, 4, 6, 8];
        merge(&mut one, 3, &mut two, 3);
        
        print!("\n{:?}\n", one);
        assert_eq!(one.len(), 8);
        assert_eq!(one, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
    
    #[test]
    fn e7() {
        let mut one = vec![1, 2, 3, 0, 0, 0];
        let mut two = vec![2, 5, 6];
        merge(&mut one, 3, &mut two, 3);

        print!("\n{:?}\n", one);
        assert_eq!(one.len(), 6);
        assert_eq!(one, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn e8() {
        let mut one = vec![4,0,0,0,0,0];
        let mut two = vec![1,2,3,5,6];
        merge(&mut one, 1, &mut two, 5);

        print!("\n{:?}\n", one);
        assert_eq!(one.len(), 6);
        assert_eq!(one, vec![1, 2, 3, 4, 5, 6]);
    }
}
