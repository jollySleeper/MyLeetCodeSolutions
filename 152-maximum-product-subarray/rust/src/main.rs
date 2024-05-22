pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut final_product = i32::MIN;
    let mut product_sig = 1;
    let mut product_usig = 1;

    for index in 0..nums.len() {
        product_sig *= nums[index];
        product_usig *= nums[index];

        let mut tmp = 0;
        if product_usig >= product_sig {
            tmp = product_usig;
        }
        if product_usig < product_sig {
            tmp = product_sig;
        }
        if final_product < tmp {
            final_product = tmp;
        }

        if product_usig <= 0 {
            product_usig = 1
        }
    }

    return final_product;
}

fn main() {
    println!("Hello, world!");

    let nums = vec![2, 3, -2, 4];
    let nums2 = Vec::from([-3, -1, -1]);
    let nums3 = Vec::from([3, -1, 4]);
    let nums4 = Vec::from([2, -5, -2, -4, 3]);

    println!("Answer {:?}", max_product(nums4));
}
