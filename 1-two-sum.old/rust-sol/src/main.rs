/**
 * Two Sum
 * https://leetcode.com/problems/two-sum/
 */

#[allow(dead_code)]

// O(n2) solution
fn two_sum_n2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for index in 0..nums.len() {
        // println!("Index:{}, Value:{}", index, nums[index]);
        for jndex in (index + 1)..nums.len() {
            if nums[index] + nums[jndex] == target {
                result.push(index.try_into().unwrap());
                result.push(jndex.try_into().unwrap());
            }
        }
    }

    return result;
}

// O(n) solution
use std::collections::HashMap;
fn two_sum_n(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    // map[&nums[0]] = 0;
    map.insert(nums[0], 0);
    for index in 1..nums.len() {
        if map.contains_key(&(target - nums[index])) {
            result.push(map[&(target - nums[index])]);
            result.push(index.try_into().unwrap());
            break;
        }
        map.insert(nums[index], index.try_into().unwrap());
    }

    return result;
}

fn main() {
    println!("Hello, world!");

    let nums: Vec<i32> = Vec::from([2, 7, 11, 15]);

    println!("Result {:?}", two_sum_n(nums, 17));
}
