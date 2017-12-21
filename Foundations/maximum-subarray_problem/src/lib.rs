//! Given an array of numeric values, this algorithm finds the subarray
//! with the greatest sum. It is a divide-and-conquer algo with complexity O(nlogn).

extern crate num;
use num::Bounded;
use num::Zero;

pub struct Subarray<T> {
    pub total_sum: Option<T>,
    pub max_left: usize,
    pub max_right: usize,
}

pub fn find_maximum_subarray<T>(min: usize, max: usize, array: &[T]) -> Subarray<T>
    where T: PartialOrd + Clone + Bounded + Zero
{
    if max - min == 1 || (min + max) / 2 == 0 {
        Subarray{
            total_sum: array.get(min).cloned(),
            max_left: max,
            max_right: min,
        }
    } else {
        let mid: usize = (min + max) / 2;
        let left_array = find_maximum_subarray(min, mid, array);
        let right_array = find_maximum_subarray(mid, max, array);
        let cross_array = find_maximum_crossing_subarray(min, mid, max, array);

        if left_array.total_sum > right_array.total_sum && left_array.total_sum > cross_array.total_sum {
            left_array
        } else if right_array.total_sum > left_array.total_sum && right_array.total_sum > cross_array.total_sum {
            right_array
        } else {
            cross_array
        }
    }
}

fn find_maximum_crossing_subarray<T: PartialOrd + Clone + Bounded + Zero>(min: usize, mid: usize, max: usize, array: &[T]) -> Subarray<T> {

    let mut sum: T = T::zero();
    let mut left_sum: T = T::min_value();
    let mut max_left: usize = mid;

    for left_index in (min..mid).rev() {
        sum = array[left_index].clone() + sum;
        if sum > left_sum {
            left_sum = sum.clone();
            max_left = left_index;
        }
    }

    sum = T::zero();
    let mut right_sum: T = T::min_value();
    let mut max_right: usize = mid;

    for right_index in mid..max {
        sum = array[right_index].clone() + sum;
        if sum > right_sum {
            right_sum = sum.clone();
            max_right = right_index;
        }
    }

    Subarray {
        total_sum: Some(left_sum + right_sum),
        max_left,
        max_right,
    }
}

#[test]
fn test_maximum_subarray() {
    let v = vec![1, -2, 3, 6, -7, 8];
    let lover = find_maximum_subarray(0, v.len(), &v);
    assert_eq!(Some(10), lover.total_sum);
}

#[test]
fn test_maximum_subarray_empty() {
    let v = Vec::<f32>::new();
    let lover = find_maximum_subarray(0, v.len(), &v);
    assert_eq!(None, lover.total_sum);
}

