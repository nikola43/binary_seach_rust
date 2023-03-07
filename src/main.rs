fn binary_seach(arr: Vec<i32>, l: i32, r: i32, x: i32) -> i32 {
    if r >= l {
        let mid: i32 = l + (r - l) / 2;

        // If the element is present at the middle
        // itself
        if arr[mid as usize] == x {
            return mid;
        }

        // If element is smaller than mid, then
        // it can only be present in left subarray
        if arr[mid as usize] > x {
            return binary_seach(arr, l, mid - 1, x);
        }

        // Else the element can only be present
        // in right subarray
        return binary_seach(arr, mid + 1, r, x);
    }

    // We reach here when element is not
    // present in array
    return -1;
}

fn main() {
    let arr = vec![56, 23, 44, 34, 56, 73, 23, 4];
    let n = arr.len();
    let x = 34;
    let result = binary_seach(arr, 0, (n - 1).try_into().unwrap(), x);

    if result > -1 {
        println!("Element {} found at index: {}", x, result)
    } else {
        println!("Not found")
    }
}
