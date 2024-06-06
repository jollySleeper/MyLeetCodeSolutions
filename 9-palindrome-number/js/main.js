/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function(x) {
    if (x < 0) {
        return false;
    }
    if (x == 0) {
        return  true;
    }

    let nums = [];
    while (x > 0) {
        nums.push(x % 10);
        x = x/10;
        x = Math.floor(x);
    }

    for (let index = 0; index < (nums.length / 2); index++) {
        if (nums[index] != nums[nums.length - 1 - index]) {
            return false;
        }
    }

    return true;
};

let num = 12011;
console.log(`Number '${num}' is Palindrome = ${isPalindrome(num)}`);
