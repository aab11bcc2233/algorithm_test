use std::cmp::PartialOrd;

mod bubble_sort;

fn swap<T>(list: &mut [T], l_index: usize, r_index: usize)
where
    T: PartialOrd + Copy,
{
    let temp = list[r_index];
    list[r_index] = list[l_index];
    list[l_index] = temp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_bubble_sort() {
        let mut list = [8, 100, 50, 22, 15, 6, 1, 1000, 999, 0];
        bubble_sort::sort(&mut list[..]);
        //        list[0] = 2345;
        println!("{:?}", list);
    }
}
