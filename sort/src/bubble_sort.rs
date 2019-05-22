use std::cmp::PartialOrd;
use crate::swap;

pub fn sort<T>(list: &mut [T])
    where T: PartialOrd + Copy {
    let size = list.len();

    for i in 0..size - 1 {
        for j in 0..(size - 1) - i {
            let l_index = j;
            let r_index = j + 1;
            if list[l_index] > list[r_index] {
                swap(list, l_index, r_index);
            }
        }
    }
}
