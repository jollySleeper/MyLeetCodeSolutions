fn next_permutation(nums: &mut Vec<i32>) {
    let mut copy_nums = nums.clone();
    copy_nums.sort();

    let mut permutations: Vec<Vec<i32>> = Vec::new();

    produce_permutations(0, &mut copy_nums, &mut permutations);
    println!("All Permutations = {:?}", permutations);

    for i in 0..permutations.len() {
        if permutations[i].clone() == nums.clone() {
            for j in 0..nums.clone().len() {
                nums[j] = permutations[i + 1][j];
            }
            return;
        }
    }
}

fn produce_permutations(index: usize, nums: &mut Vec<i32>, permutations: &mut Vec<Vec<i32>>) {
    if index >= nums.len() {
        permutations.push(nums.clone());
        return;
    }

    for i in index..nums.len() {
        nums.swap(index, i);
        produce_permutations(index + 1, nums, permutations);
        nums.swap(i, index);
    }
}

fn main() {
    println!("Hello, world!");

    let mut nums = Vec::from([1, 2, 3]);
    println!("Nums/Initail Permutation = {:?}", nums);
    next_permutation(&mut nums);
    println!("Next Permutation = {:?}", nums);
}
