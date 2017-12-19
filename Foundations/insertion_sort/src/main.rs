/*
Insertion sort runs in  O(n^2) time. The idea of insertion
sort is to "sort in place", meaning it rearranges the values within
the passed array. With each loop pass, the sorted list grows by one.
LOOP INVARANT: At the start of each iteration of the ouer loop, the
elements in the range (0, i-1) are in sorted order and consist of
the elements that were originally in (0, i-1) when the algo started.
*/

fn main() {
    let mut array: Vec<f32> = vec![3.,3.,5.,2.,1.];
    insertion_sort(&mut array);
    println!("{:?}",array);
}

fn insertion_sort(array: &mut Vec<f32>) {
    for i in 1..array.len() {
        let key = array[i];
        let mut j: usize = i - 1;
        while j >= 0 && array[j] > key {
            array[j+1] = array[j];
            match j {
                0 => break,
                _ => j -= 1,
            }
        }
        array[j] = key;
    }
}


