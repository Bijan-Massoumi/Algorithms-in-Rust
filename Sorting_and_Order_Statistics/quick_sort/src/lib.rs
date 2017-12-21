
fn quicksort<T: std::cmp::PartialOrd + Clone>(p: usize, r: usize,  array: &mut [T]) {
    if p < r {
        let pivot = partition(p, r, array);
        quicksort(p, pivot - 1, array);
        quicksort(pivot + 1, r, array)
    }
}

fn partition<T: std::cmp::PartialOrd + Clone>(p: usize, r: usize, array: &mut [T]) -> usize {
    let pivot: T = array[r].clone();
    let mut i = (p as isize - 1);
    for k in p..r {
        if array[k] <= pivot {
            i += 1;
            array.swap(i as usize,k);
        }
    }
    array.swap((i+1) as usize, r);

    (i + 1) as usize
}


#[test]
fn test_quicksort() {
    let mut test = vec![1,9,6,7,4,5,3,5,4,6,7,8,9];
    quicksort(0, test.len() - 1, &mut test);
    assert_eq!(test,[1, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 9, 9]);
}

#[test]
fn test_quicksort_empty() {
    let mut v = Vec::<f32>::new();
    quicksort(0, v.len(), &mut v);
    assert_eq!(v, Vec::<f32>::new());
}