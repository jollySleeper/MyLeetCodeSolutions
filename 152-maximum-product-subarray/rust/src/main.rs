use std::cmp::max;

pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut final_product = i32::MIN;
    let mut product_prefix = 1;
    let mut product_suffix = 1;

    for index in 0..nums.len() {
        product_prefix *= nums[index];
        product_suffix *= nums[nums.len() - 1 - index];

        final_product = max(final_product, max(product_prefix, product_suffix));
        if product_prefix == 0 {
            product_prefix = 1;
        }
        if product_suffix == 0 {
            product_suffix = 1;
        }
    }

    return final_product;
}

fn main() {
    println!("Hello, world!");

    let _nums = vec![2, 3, -2, 4];
    let _nums2 = Vec::from([-3, -1, -1]);
    let _nums3 = Vec::from([3, -1, 4]);
    let nums4 = Vec::from([2, -5, -2, -4, 3]);

    println!("Answer {:?}", max_product(nums4));
}
