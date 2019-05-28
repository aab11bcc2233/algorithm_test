use crate::*;
use std::cmp::PartialOrd;

// 将 list[j] 与 list[0]-list[j-1] 之间的数做比较，放到合适的位置。
pub fn sort<T>(list: &mut [T])
    where
        T: PartialOrd + Copy,
{
    let len = list.len();

    for i in 0..len {

        let mut min = i;
        for j in i+1..len {
            if less(list[j], list[min]) {
                min = j;
            }
        }

        swap(list, i, min);
    }
}