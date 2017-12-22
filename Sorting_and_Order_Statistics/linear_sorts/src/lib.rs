#[cfg(test)]

// This file includes count_sort, and bucket_sort


mod linear_sorts {
    mod count_sort {
        pub fn count_sort(array: &[usize], k: usize)  -> Vec<usize>{

            let mut count_array: Vec<usize> = (0..k + 1).map(|_| 0).collect();
            let mut sorted_array: Vec<usize> = (0..array.len()).map(|_|0).collect();

            for i in 0..array.len() {
                count_array[array[i]] += 1;
            }

            for i in 1..count_array.len() {
                count_array[i] += count_array[i-1]
            }

            for i in (0..array.len()).rev() {
                sorted_array[count_array[array[i]] - 1] = array[i];
                count_array[array[i]] -= 1;
            }
            sorted_array
        }

        #[test]
        fn test_count_sort() {
            let test = vec![1,2,4,3,6,7];
            let love = count_sort(&test,7);
            println!("{:?}", love);
            assert_eq!(love, [1,2,3,4,6,7])
        }
    }

    mod bucket_sort {
        pub fn bucket_sort(array: &[usize], k: usize){
            // will implement after I finish implementing linkedlists
        }
    }
}