use crate::*;

pub fn sort(list: &mut [i32]) {
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