use std::cmp::PartialOrd;

mod bubble_sort;
mod quick_sort;

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
    fn test_bubble_sort() {
        let mut list = [8, 100, 50, 22, 15, 6, 1, 1000, 999, 0];
        println!("test sort before -> {:?}", list);
        bubble_sort::sort(&mut list[..]);
        println!("test sort after -> {:?}", list);
    }

    #[test]
    fn test_quick_sort() {
        let mut list = [6, 1, 2, 7, 9, 3, 4, 5, 10, 8];
        println!("test sort before -> {:?}", list);
        //        let mut list = [6, 1];
        quick_sort::sort(&mut list[..]);
        println!("test sort after -> {:?}", list);
    }
}
