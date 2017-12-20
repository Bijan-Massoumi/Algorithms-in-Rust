/*
the complexity is O(nlogn). It is a divide and
conquor algo with recurrence destrubed as so
T(n) = 2*T(n/2) + O(n).
*/


fn main() {
    let mut love: Vec<f32> = vec![8.,9.,1.,4.,10.];
    merge_sort(0,love.len(),&mut love);
    println!("{:?}",love);
}


fn merge_sort( p: usize, r: usize, array: &mut Vec<f32>) {
    if p < r {
        let q = (p + r)/2;
        merge_sort(p, q, array);
        merge_sort( q + 1, r, array);
        merge(p, q, r, array);
    }
}

fn merge(p: usize, q: usize, r: usize, array: &mut Vec<f32>) {
    let left_size: usize = q - p;
    let right_size: usize = r - q;

    let mut left: Vec<f32> = Vec::new();
    let mut right: Vec<f32> = Vec::new();

    for i in 0..left_size {
        left.push(array[p + i]);
    }

    for i in 0..right_size {
        right.push(array[q + i]);
    }

    left.push(1000000.);
    right.push(1000000.);

    let mut i: usize = 0;
    let mut j: usize = 0;

    for k in p..r {
        if left[i] <= right[j] {
            array[k] = left[i];
            i += 1;
        } else {
            array[k] = right[j];
            j += 1;
        }
    }
}
