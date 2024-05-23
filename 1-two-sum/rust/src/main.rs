use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(nums[0], 0);
    for index in 1..nums.len() {
        let needed: i32 = target - nums[index];
        if map.contains_key(&needed) {
            return [index.try_into().unwrap(), map[&needed]].to_vec();
        }
        map.insert(nums[index], index.try_into().unwrap());
    }

    return [-1, -1].to_vec();
}

fn main() {
    println!("Hello, world!");

    let nums = Vec::from([2, 7, 11, 15]);
    let target = 9;
    println!("Result is = {:?}", two_sum(nums, target));
}
