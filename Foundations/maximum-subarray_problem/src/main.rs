
/*
Given an array of numeric values, this algorithm finds the subarray
with the greatest sum. It is a divide-and-conquer algo with complexity O(nlogn).
*/

extern crate num;
use num::Bounded;
use num::Zero;


struct Subarray<T> {
    total_sum: T,
    max_left: usize,
    max_right: usize,
}

fn main() {
    let v: Vec<f32> = vec![1., -2., 3., 6., -7., 8.];
    let lover = find_maximum_subarray(0, v.len(), &v );
    println!("{} and {} and {}", lover.max_left, lover.max_right, lover.total_sum);

}


fn find_maximum_subarray<T:  PartialOrd + Copy + Bounded + Zero> (min: usize, max: usize, array: &[T] ) -> Subarray<T> {
    if max - min ==  1 {
        return Subarray{
            total_sum: array[min],
            max_left: max,
            max_right: min,
        }
    } else {
        let mid: usize = (min + max)/2;
        let left_array = find_maximum_subarray(min, mid, array);
        let right_array = find_maximum_subarray(mid, max, array);
        let cross_array = find_maximum_crossing_subarray(min, mid, max, array);

        if (left_array.total_sum > right_array.total_sum && left_array.total_sum > cross_array.total_sum) {
            return left_array;
        } else if (right_array.total_sum > left_array.total_sum && right_array.total_sum > cross_array.total_sum) {
            return right_array;
        } else {
            return cross_array;
        }
    }
}

fn find_maximum_crossing_subarray<T: PartialOrd + Copy + Bounded + Zero>( min: usize, mid: usize, max: usize, array: &[T] ) -> Subarray<T> {
    let mut sum: T = T::zero();
    let mut left_sum: T = T::min_value();
    let mut max_left: usize = mid;

    for left_index in (min..mid).rev() {
        sum = array[left_index] + sum;
        if sum > left_sum {
            left_sum = sum;
            max_left = left_index;
        }
    }

    sum = T::zero();
    let mut right_sum: T = T::min_value();
    let mut max_right: usize = mid;

    for right_index in mid..max {
        sum = array[right_index] + sum;
        if sum > right_sum {
            right_sum = sum;
            max_right = right_index;
        }
    }

    return Subarray {
        total_sum: left_sum + right_sum,
        max_left,
        max_right,
    };
}