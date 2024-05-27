use std::collections::HashMap;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map: HashMap<i32, bool> = HashMap::new();
    map.insert(nums[0], true);

    for index in 1..nums.len() {
        if map.contains_key(&nums[index]) == true {
            return true;
        }
        map.insert(nums[index], true);
    }

    return false;
}

fn main() {
    let nums = Vec::from([1, 2, 3, 1]);
    println!("Nums = {:?}", nums);
    println!("Contains Duplicate = {}", contains_duplicate(nums));
}
