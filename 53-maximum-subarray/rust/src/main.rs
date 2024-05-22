/**
 * Maximum Subarray
 * https://leetcode.com/problems/maximun-subarray
 */

#[allow(dead_code)]

fn kadane(nums: Vec<i32>) -> i32 {
    if nums.len() < 1 {
        return -1;
    }

    let mut sum: i32 = 0;
    let mut temp_sum: i32 = 0;
    for index in 0..nums.len() {
        // println!("Index:{}, Value:{}", index, nums[index]);
        temp_sum += nums[index];
        if sum < temp_sum {
            sum = temp_sum;
        }
        if temp_sum < 0 {
            temp_sum = 0;
        }
    }

    return sum;
}

fn main() {
    println!("Hello, world!");

    let nums: Vec<i32> = Vec::from([-2, 1, -3, 4, -1, 2, 1, -5, 4]);

    println!("Result {:?}", kadane(nums));
}
