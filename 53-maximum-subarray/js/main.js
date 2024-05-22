/**
 * O(n) Solution
 *
 * @param {number[]} nums
 * @return {number}
 */
let maxSubArray = function(nums) {
    let finalSum = Number.MIN_SAFE_INTEGER;
    let sum = 0;

    for (let index = 0; index < nums.length; index++) {
        sum += nums[index];
        if (sum > finalSum) {
            finalSum = sum;
        }
        if (sum < 0) {
            sum = 0
        }
    }

    return finalSum;
};

let nums = [-2,1,-3,4,-1,2,1,-5,4];
console.log("Sum is = ", maxSubArray(nums));
