
fn max_heapify<T: PartialOrd >(i: usize, heap_size: usize, array: &mut [T]) {
    let left: usize = (i * 2) + 1;
    let right: usize = (i * 2) + 2;
    let mut largest: usize = i;

    if left < heap_size && array[left] > array[largest] {
        largest = left;
    }
    if right < heap_size && array[right] > array[largest] {
        largest = right;
    }

    if largest != i {
        array.swap(largest,i);
        max_heapify(largest, heap_size, array);
    }
}

fn build_max_heap<T:  PartialOrd  >(array: &mut [T]) {
    for i in (0.. (array.len() / 2)  ).rev() {
        max_heapify(i,array.len(), array);
    }
}

fn heap_sort<T: PartialOrd >(array: &mut [T] ) {
    build_max_heap(array);
    for i in (1..array.len()).rev() {
        array.swap(0, i);
        max_heapify(0, i, array);
    }
}


#[test]
fn test_heap_sort() {
    let mut love = vec![1,2,3,4,5,6,7,8,9,66,43,11,2,7,9,5,4,77,66,45,34];
    heap_sort(&mut love);
    assert_eq!(love, [1, 2, 2, 3, 4, 4, 5, 5, 6, 7, 7, 8, 9, 9, 11, 34, 43, 45, 66, 66, 77]);
}

#[test]
fn test_heap_sort_empty() {
    let mut love = Vec::<f32>::new();
    heap_sort(&mut love);
    assert_eq!(love, Vec::<f32>::new());

}