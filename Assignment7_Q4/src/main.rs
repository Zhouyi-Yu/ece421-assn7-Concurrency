// rayon::join is used directly via the crate name or can be imported.

fn main() {
    let mut v = vec![10, 5, 8, 1, 6, 3, 9, 2, 4, 7];
    println!("Before sorting: {:?}", v);
    concurrent_quick_sort(&mut v);
    println!("After sorting: {:?}", v);
}

fn concurrent_quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    // If the slice has 1 or fewer elements, it is already sorted
    if v.len() <= 1 {
        return;
    }

    // Partition the slice and get the index of the pivot
    let pivot_index = partition(v);

    // Split the slice into two parts, excluding the pivot element
    let (left, right) = v.split_at_mut(pivot_index);
    let right_after_pivot = &mut right[1..];

    // Use rayon::join to sort both halves concurrently
    rayon::join(
        || concurrent_quick_sort(left),
        || concurrent_quick_sort(right_after_pivot),
    );
}

fn partition<T: PartialOrd>(v: &mut [T]) -> usize {
    let len = v.len();
    let pivot_idx = len - 1;
    let mut i = 0;

    for j in 0..pivot_idx {
        if v[j] <= v[pivot_idx] {
            v.swap(i, j);
            i += 1;
        }
    }

    v.swap(i, pivot_idx);
    i
}
