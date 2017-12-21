/*
heap_sort runs in O(nlogn). its made data structure is a heap, which
acts as a flattened binary tree which can be either max heap witht he propery that
parent(i) >= i and min being similar. How heapify works is that it takes an index,
and puts the greatest value of its subtree at the root, and puts the old misplaced root into
the correct place in its subtree. So it build a full heap, you go from bottom to top (array.len /2)
calling heapify at every index. heap_sort, take the root of the whole heap (which should be the greatest
value) and puts it at the end of the array, lowers the size of the heap which is less than the size of the
array, then does the same thing recursivly until we hit the top of the tree.
*/




fn main() {
    let mut love = vec![1,2,3,4,5,6,7,8,9,66,43,11,2,7,9,5,4,77,66,45,34];
    heap_sort(&mut love);
    println!("{:?}", love);
}

fn max_heapify<T: PartialOrd + Copy>(i: usize, heap_size: usize, array: &mut Vec<T>) {
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

fn build_max_heap<T:  PartialOrd + Copy >(array: &mut Vec<T>) {
    for i in (0.. (array.len() / 2)  ).rev() {
        max_heapify(i,array.len(),array);
    }
}

fn heap_sort<T: PartialOrd + Copy>(array: &mut Vec<T> ) {
    build_max_heap(array);
    for i in (1..array.len()).rev() {
        array.swap(0,i);
        max_heapify(0, i, array);
    }
}