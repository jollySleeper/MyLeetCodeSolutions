/**
 * @param {number[]} nums
 * @return {number[][]}
 */
let permute = function(nums) {
    let permutations = [];
    let current_permutation = [];
    let number_used_map = [];
    for (let index = 0; index < nums.length; index++) {
        number_used_map[index] = false;
    }

    recursive_permutation(nums, current_permutation, number_used_map, permutations);

    return permutations;
};

function recursive_permutation(nums, current_permutation, number_used_map, permutations) {
    if (current_permutation.length === nums.length) {
        // console.log("Current Permutation = " + current_permutation);
        // NOTE: Use concat function so that values are copied
        permutations.push(current_permutation.concat());
        // console.log(permutations);

        return;
    }

    for (let index = 0; index < nums.length; index++) {
        if (!number_used_map[index]) {
            number_used_map[index] = true;
            current_permutation.push(nums[index]);

            recursive_permutation(nums, current_permutation, number_used_map, permutations);

            current_permutation.pop();
            number_used_map[index] = false;
        }
    }
}


let nums = [1,2,3];
console.log("Permutations = ");
console.log(permute(nums));
