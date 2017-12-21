/*
Insertion sort runs in  O(n^2) time. The idea of insertion
sort is to "sort in place", meaning it rearranges the values within
the passed array. With each loop pass, the sorted list grows by one.
LOOP INVARANT: At the start of each iteration of the ouer loop, the
elements in the range (0, i-1) are in sorted order and consist of
the elements that were originally in (0, i-1) when the algo started.
*/

pub fn insertion_sort<T: PartialOrd + Clone>(array: &mut [T]) {

    if array.len() < 1 { return; }

    for i in 0..(array.len() - 1) {

        let key = array[(i + 1)].clone();
        let mut j = i as isize;

        while j >= 0 && array[j as usize] > key {
            array[(j+1) as usize] = array[j as usize].clone();
            j -= 1;
        }

        array[(j + 1) as usize] = key;
    }
}


#[test]
fn test_insertion_sort() {
    let mut unsorted = [5, 6, 3, 9, 8];
    insertion_sort(&mut unsorted);
    assert_eq!([3, 5, 6, 8, 9], unsorted);
}

#[test]
fn test_insertion_sort_empty() {
    let mut unsorted = Vec::<u32>::new();
    insertion_sort(&mut unsorted);
    assert_eq!(Vec::<u32>::new(), unsorted);
}
