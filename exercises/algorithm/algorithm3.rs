/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T]) where T: Ord {
	//TODO
     array.sort();
}

// fn sort<T>(array: &mut [T])
// where
//     T: Ord,
// {
//     if array.len() <= 1 {
//         return;
//     }
    
//     quicksort_helper(array);
// }

fn quicksort_helper<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    
    let pivot_idx = partition(array);
    let (left, right) = array.split_at_mut(pivot_idx);
    quicksort_helper(left);
    quicksort_helper(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_idx = arr.len() - 1;
    let mut i = 0;
    
    for j in 0..pivot_idx {
        if arr[j] <= arr[pivot_idx] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, pivot_idx);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}