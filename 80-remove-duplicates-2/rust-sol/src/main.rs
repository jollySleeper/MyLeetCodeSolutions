/**
 * Remove Duplicates From Sorted Array 2
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
 */

#[allow(dead_code)]

fn remove_duplicate_n_wrong(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len().try_into().unwrap();
    }

    let mut i = 0;
    let mut j = 2;
    let mut max_index_traveresed = 0;

    while i < nums.len() && j < nums.len() {
        if nums[i] == nums[j] {
            println!(
                "Woops Equal, Nums={:?}, i={}, j={}, max_index_traveresed={}",
                nums, i, j, max_index_traveresed
            );
            for k in (max_index_traveresed + 1)..nums.len() {
                max_index_traveresed = k;
                if nums[i] != nums[k] {
                    // Copy Value
                    nums[j] = nums[k];
                    break;
                }
            }
        }

        if j > max_index_traveresed {
            max_index_traveresed = j;
        }

        println!(
            "Nums={:?}, i={}, j={}, max_index_traveresed={}",
            nums, i, j, max_index_traveresed
        );

        if (max_index_traveresed + 1) == nums.len() {
            return (i + 2).try_into().unwrap();
        }

        i += 1;
        j += 1;
    }

    return (i + 2).try_into().unwrap();
}

// 1, 1, 1, 2, 2, 3
// 1, 1, 2, 2, 2, 3  max_index_traveresed = 3
// 1, 1, 2, 2, 2, 3  max_index_traveresed = 3
// 1, 1, 2, 2, 3, 3  max_index_traveresed = 5

// 0, 0, 1, 1, 1, 1, 2, 3, 3, 4, 4, 4
// 0, 0, 1, 1, 2, 1, 2, 3, 3, 4, 4, 4 max_index_traveresed = 6
// 0, 0, 1, 1, 2, 3, 2, 3, 3, 4, 4, 4 max_index_traveresed = 7
// 0, 0, 1, 1, 2, 3, 3, 3, 3, 4, 4, 4 max_index_traveresed = 8
// 0, 0, 1, 1, 2, 3, 3, 3, 4, 4, 4, 4 max_index_traveresed = 8

// 0, 0, 1, 1, 1, 1, 2, 3, 3
// 0, 0, 1, 1, 2, 1, 2, 3, 3 i2 j4 max_index_traveresed = 6
// 0, 0, 1, 1, 2, 3, 2, 3, 3 i3 j5 max_index_traveresed = 7
// 0, 0, 1, 1, 2, 3, 3, 3, 3 i4 j6 max_index_traveresed = 8

fn remove_duplicate_n(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len().try_into().unwrap();
    }

    let mut i = 2;
    let mut j = 2;
    while j < nums.len() {
        if nums[j] != nums[j - 2] {
            nums[i] = nums[j];
            i += 1;
        }
        j += 1;
    }

    return (i).try_into().unwrap();
}

fn main() {
    println!("Hello, world!");
    let mut nums: Vec<i32> = Vec::from([1, 1, 1, 2, 2, 3]);
    // let mut nums: Vec<i32> = Vec::from([0, 0, 1, 1, 1, 1, 2, 3, 3]);
    // let mut nums: Vec<i32> = Vec::from([0, 0, 1, 1, 1, 1, 2, 3, 3, 4, 4, 4]);
    // let mut nums: Vec<i32> = Vec::from([1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3]);

    println!("Result {:?}", remove_duplicate_n(&mut nums));
}
