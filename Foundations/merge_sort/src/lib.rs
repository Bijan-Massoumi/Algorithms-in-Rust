//! The complexity is O(nlogn). It is a divide and
//! conquor algo with recurrence destrubed as so
//! `T(n) = 2*T(n/2) + O(n)`.

extern crate num;
use num::Bounded;

pub fn merge_sort<T: PartialOrd + Clone + Bounded>( p: usize, r: usize, array: &mut [T]) {
    if p < r {
        let q = (p + r)/2;
        merge_sort(p, q, array);
        merge_sort( q + 1, r, array);
        merge(p, q, r, array);
    }
}

fn merge<T: PartialOrd + Clone + Bounded>(p: usize, q: usize, r: usize, array: &mut [T]) {
    let left_size = q - p;
    let right_size = r - q;

    let mut left: Vec<T>  = (0..left_size).map(|i| array[p + i].clone()).collect();
    let mut right: Vec<T> = (0..right_size).map(|i| array[q + i].clone()).collect();

    left.push(T::max_value());
    right.push(T::max_value());

    let mut i: usize = 0;
    let mut j: usize = 0;

    for k in p..r {
        if left[i] <= right[j] {
            array[k] = left[i].clone();
            i += 1;
        } else {
            array[k] = right[j].clone();
            j += 1;
        }
    }
}

#[test]
fn test_merge_sort() {
    let mut love = [8, 9, 1, 4, 10];
    merge_sort(0,love.len(),&mut love);
    assert_eq!(love, [1, 4, 8, 9, 10]);
}

#[test]
fn test_merge_sort_empty() {
    let mut love = Vec::<u32>::new();
    merge_sort(0,love.len(),&mut love);
    assert_eq!(love, Vec::<u32>::new());
}
