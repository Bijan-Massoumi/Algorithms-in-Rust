/*
Insertion sort runs in  O(n^2) time. The idea of insertion
sort is to "sort in place", meaning it rearranges the values within
the passed array. With each loop pass, the sorted list grows by one.
LOOP INVARANT: At the start of each iteration of the ouer loop, the
elements in the range (0, i-1) are in sorted order and consist of
the elements that were originally in (0, i-1) when the algo started.
*/

use std::cmp::PartialOrd;

fn main() {
    let mut array: Vec<f32> = vec![3.,2.,5.5,2.,1.,8., 6.,3.];
    insertion_sort(&mut array);
    println!("{:?}",array);
}

fn insertion_sort<T: PartialOrd + Copy + std::fmt::Debug>(array: &mut Vec<T>) {
    for i in 1..array.len() as isize {
        let key = array[i as usize];
        let mut j: isize = (i - 1) ;
        while j >= 0 && array[j as usize] > key {
            array[(j+1) as usize] = array[j as usize];
            j -= 1;
        }
        array[(j + 1) as usize] = key;
    }
}


