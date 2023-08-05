/**
 * 3 Sum
 * https://leetcode.com/problems/3sum/
 */
use std::collections::HashMap;
fn three_sum_n2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut triplet_map: HashMap<Vec<i32>, bool> = HashMap::new();

    nums.sort();
    for i in 1..nums.len() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for j in 0..nums.len() {
            if j < i {
                map.insert(nums[j], j.try_into().unwrap());
            } else if j > i {
                let sum_to_zero: i32 = (nums[i] + nums[j]) * -1;
                if map.contains_key(&sum_to_zero) {
                    let triplet: Vec<i32> = Vec::from([nums[i], nums[j], sum_to_zero]);

                    if !triplet_map.contains_key(&triplet) {
                        triplet_map.insert(triplet, true);
                        result.push([nums[i], nums[j], sum_to_zero].to_vec());
                    }
                }
            }
        }
    }

    return result;
}

fn main() {
    println!("Hello, world!");

    let nums: Vec<i32> = Vec::from([-1, 0, 1, 2, -1, -4]);

    println!("Result {:?}", three_sum_n2(nums));
}
