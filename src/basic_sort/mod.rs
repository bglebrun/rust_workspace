use std::{Vec, mem};

struct search<T> {
    arr: Vec<T>,
    len: usize
}

impl<T> search<T> {
    fn new(arr: Vec<T>) -> search<T> {
        search {
            arr: arr,
            len: arr.len()
        }
    }
}

// O(n^2) in [current year]
fn bubble_sort(arr: Vec<i32>) -> Vec<i32> {
    for i in 0 .. arr.Len() {
        for j in 0 .. arr.Len() - i - 1 {
            if arr[j] > arr[j + 1] {
                // perform a memswap of the elements
                mem.swap(&mut arr[j], &mut arr[j + 1]);
            }
        }
    }
    // not the "rust" way to do this but we're practicting
    arr
}

// merge sort
fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    // base case
    if arr.len() <= 1 {
        return arr;
    }
    // recursive case
    let mid = arr.len() / 2;
    let mut left = merge_sort(arr[..mid].to_vec());
    let mut right = merge_sort(arr[mid..].to_vec());
    let mut sorted = Vec::with_capacity(arr.len());
    while !left.is_empty() && !right.is_empty() {
        if left[0] < right[0] {
            sorted.push(left.remove(0));
        } else {
            sorted.push(right.remove(0));
        }
    }
    sorted.extend(left);
    sorted.extend(right);
    sorted
}