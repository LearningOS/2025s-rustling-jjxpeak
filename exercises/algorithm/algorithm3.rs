/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
    //TODO: 插入排序
    // let mut swapped = true;
    // while swapped {
    //     swapped = false;
    //     for i in 0..array.len() - 1 {
    //         if array[i] > array[i + 1] {
    //             array.swap(i, i + 1);
    //             swapped = true;
    //         }
    //     }
    // }
    quick_sort(array, 0, array.len() - 1);
}

fn quick_sort<T: PartialOrd + Copy>(array: &mut [T], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let pivot_index = partition(left, right, array);
    if pivot_index > 0 {
        quick_sort(array, left, pivot_index - 1);
    }

    quick_sort(array, pivot_index + 1, right);
}

fn partition<T: PartialOrd + Copy>(left: usize, right: usize, array: &mut [T]) -> usize {
    let part = array[right];
    let mut i = left;
    let mut j = right;
    while i < j {
        while i < j && array[i] < part {
            i += 1;
        }
        while i < j && array[j] > part {
            j -= 1;
        }
        array.swap(i, j);
    }
    return i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        println!("{:?}", vec);
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
