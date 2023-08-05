/**
 * Remove Duplicates From Sorted Array
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array/
 */

#[allow(dead_code)]

// O(x) Solution
fn remove_duplicate_n(nums: &mut Vec<i32>) -> i32 {
    // let temp: i32 = nums[0];
    let mut num_to_compare: (i32, i32) = (nums[0], 0);
    let mut num_to_replace: i32 = i32::MIN;

    let mut i = 0;
    while i < nums.len() {
        if num_to_compare.0 == nums[i] {
            num_to_replace = (nums[i], i.try_into().unwrap());
        } else {
            if num_to_replace.0 == num_to_compare.0 {}
            num_to_compare = (nums[i], i.try_into().unwrap());
        }
        i += 1;
    }

    return 5;
}

/**
 * 0 0 1 1 1 2 2 3 3 4
 * 0 0
 *     1
 * 0 1 0 1 1 2 2
 *      
 *
 *
 *
 */

fn main() {
    println!("Hello, world!");

    let mut nums: Vec<i32> = Vec::from([0, 0, 1, 1, 1, 2, 2, 3, 3, 4]);

    println!("Result {:?}", remove_duplicate_n(&mut nums));
}
