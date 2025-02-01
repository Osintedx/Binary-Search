/// Performs a binary search on a sorted array.
pub fn binary_search(arr: &[i32], target: i32, ascending: bool) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() as i32 - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_val = arr[mid as usize];

        if mid_val == target {
            return Some(mid as usize);
        } else if (mid_val < target && ascending) || (mid_val > target && !ascending) {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}