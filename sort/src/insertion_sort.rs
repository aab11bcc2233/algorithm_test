use crate::*;
use std::cmp::PartialOrd;

pub fn sort<T>(list: &mut [T])
    where T: PartialOrd + Copy,
{
    let len = list.len();

    for i in 1..len {
        for j in (i..len).rev() {
            if less(list[j], list[j-1]) {
                swap(list, j, j-1);
            }
        }
    }
}