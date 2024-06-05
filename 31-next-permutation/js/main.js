/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var nextPermutation = function(nums) {
    let indexToRevFrom = 0;
    for (let i = nums.length - 1; i >= 0; i--) {
        if (nums[i-1] > nums[i]) {
            continue;
        }
        indexToRevFrom = i - 1;
        break;
    }

    // For Reversing String
    let j = nums.length - 1;
    for (let index = indexToRevFrom; index < j; index++) {
        console.log(`index = ${index}, j = ${j}`);
        let tmp = nums[index];
        nums[index] = nums[j];
        nums[j] = tmp;
        j--;
    }

    return nums;
};

let nums = [1,2,3,5,6,4];
// NOTE: Swap then reverse

console.log('Next Permutation =' + nextPermutation(nums));
