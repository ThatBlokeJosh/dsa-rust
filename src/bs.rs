use std::fmt::Debug;

fn raw_search<T: PartialOrd + Sized + Debug>(arr: &[T], l: i32, r: i32, value: T) -> Option<usize> {

    if r >= l {
        let mut middle = r - l;
        middle /= 2;
        middle += l;

        if value == arr[middle as usize] {
            return Some(middle as usize);
        } else if arr[middle as usize] > value {
            return raw_search(&arr, l, middle-1, value);
        } else {
            return raw_search(&arr, middle+1, r, value);
        } 
    } 
    else {
        return None;
    }
}

pub fn search<T: PartialOrd + Sized + Debug>(arr: &[T], value: T) -> Option<usize> {
    let l = 0;
    let mut r = (arr.len() as i32).try_into().unwrap();
    r -= 1;
    if value < arr[l as usize] || value > arr[r as usize] {
        return None;
    } else {
        return raw_search(arr, l, r, value);
    }
} 

#[cfg(test)]
mod tests {
    #[test]
    fn working_search() {
        use super::search;
        let arr:[i32; 5] = [1,2,3,4,5];
        assert_eq!(search(&arr, 4), Some(3));
    } 
}
