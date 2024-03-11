use std::fmt::Debug;

pub fn sort<T: Debug + PartialOrd + Copy>(arr: &mut Vec<T>) {
    if arr.len() <= 1 {
        return;
    }

    let middle:usize = arr.len() / 2;
    let mut left = &arr[..middle]; 
    let mut right = &arr[middle..];
    sort(&mut left.to_vec());
    sort(&mut right.to_vec());

    let i = 0;
    let j = 0;
    let k = 0;

    while i < left.len() && j < right.len()  {
       if left[i] <= right[j] {
         arr[k] = left[i];
       } 
    }
}
