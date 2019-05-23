use crate::swap;
use std::cmp::PartialOrd;

pub fn sort<T>(list: &mut [T])
where
    T: PartialOrd + Copy,
{
    let len = list.len();
    if len <= 1 {
        return;
    }

    let value = list[0];
    let mut l_index = 0;
    let mut r_index = len;

    let mut is_right = true;

    while l_index < r_index {
        if is_right {
            r_index -= 1;

            if list[r_index] < value {
                is_right = false;
            }
        } else {
            l_index += 1;

            if list[l_index] > value {
                swap(list, l_index, r_index);
                is_right = true;
            }
        }
    }

    swap(list, 0, l_index);

    if l_index > 1 {
        sort(&mut list[..l_index]);
        sort(&mut list[l_index + 1..]);
    }
}
