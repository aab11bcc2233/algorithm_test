use std::cmp::PartialOrd;

pub mod bubble_sort;
pub mod quick_sort;
pub mod selection_sort;
pub mod insertion_sort;

pub fn swap<T>(list: &mut [T], l_index: usize, r_index: usize)
    where
        T: PartialOrd + Copy,
{
    let temp = list[r_index];
    list[r_index] = list[l_index];
    list[l_index] = temp
}

pub fn less<T>(a: T, b: T) -> bool
    where
        T: PartialOrd + Copy,
{
    a < b
}

pub fn is_sorted<T>(list: &[T]) -> bool
    where
        T: PartialOrd + Copy,
{
    for i in 1..list.len() {
        if list[i] < list[i - 1] {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_bubble_sort() {
        let mut list = [8, 100, 50, 22, 15, 6, 1, 1000, 999, 0];
//        println!("test sort before -> {:?}", list);
        bubble_sort::sort(&mut list[..]);
//        println!("test sort after -> {:?}", list);
        assert_is_sorted(&list);
    }

    #[test]
    fn test_quick_sort() {
//        let mut list = [6, 1, 2, 7, 9, 3, 4, 5, 10, 8];
//        println!("test sort before -> {:?}", list);
        let mut list = [6, 1];
        quick_sort::sort(&mut list[..]);
//        println!("test sort after -> {:?}", list);
        assert_is_sorted(&list);
    }

    #[test]
    fn test_selection_sort() {
        let mut list = [6, 1, 2, 7, 9, 3, 4, 5, 10, 8];
//        println!("test sort before -> {:?}", list);
        selection_sort::sort(&mut list[..]);
//        println!("test sort after -> {:?}", list);
        assert_is_sorted(&list);
    }

    #[test]
    fn test_insertion_sort() {
        let mut list = [6, 1, 2, 7, 9, 3, 4, 5, 10, 8];
//        println!("test sort before -> {:?}", list);
        insertion_sort::sort(&mut list[..]);
//        println!("test sort after -> {:?}", list);
        assert_is_sorted(&list);
    }

    #[test]
    fn test_is_sorted() {
        let mut list = [6, 1, 2, 7, 9, 3, 4, 5, 10, 8];
        quick_sort::sort(&mut list[..]);
//        println!("test list -> {:?}", list);
//        println!("test list is_sorted -> {}", is_sorted(&list));
    }

    fn assert_is_sorted<T>(list: &[T])
        where
            T: PartialOrd + Copy,
    {
        assert!(is_sorted(list), "list is not sorted");
    }
}
