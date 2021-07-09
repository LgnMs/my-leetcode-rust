/**
*  堆排序实现
*  时间 Ο(nlgn)
*/
use std::usize;

/// 构建最大堆
fn build_max_heap(array: &mut Vec<i32>) {
    let mut n = (array.len() - 1) / 2;
    
    loop {
        max_heap(array, n , array.len() - 1);
        if n != 0 {
            n -= 1;
        } else {
            break;
        }
    }
}

/// 最大堆实现
fn max_heap(array: &mut Vec<i32>, i: usize, heap_size: usize) {
    let l = 2 * (i + 1) - 1;
    let r = 2 * (i + 1);
    let mut largest = i;

    if l <= heap_size && array[l] > array[i] {
        largest = l;
    }
    if r <= heap_size && array[r] > array[largest] {
        largest = r;
    }
    if largest != i {
        array.swap(largest, i);
        max_heap(array, largest, heap_size);
    }
}

/// 堆排序实现
pub fn heap_sort(mut array: Vec<i32>) -> Vec<i32> {
    let mut heap_size = array.len() - 1;
    build_max_heap(&mut array);

    let mut i = array.len() - 1;
    while i > 0 {
        array.swap(0, i);
        heap_size -= 1;
        max_heap(&mut array, 0, heap_size);
        i -= 1;
    }

    array
}


#[cfg(test)]
mod tests {
    use crate::heap_sort::heap_sort;

    #[test]
    fn it_work_1() {
        assert_eq!(heap_sort(vec![3,2,1]), vec![1,2,3]);
    }
    #[test]
    fn it_work_2() {
        assert_eq!(heap_sort(vec![3,2,1,4,10,9,5,7]), vec![1,2,3,4,5,7,9,10]);
    }
}
