#include <bits/stdc++.h>

using namespace std;

// O(n^2) Solution Gives TLE
int maxProduct(vector<int>& nums) {
    if (nums.size() == 1) {
        return nums[0];
    }

    int finalProduct = INT_MIN;
    for (auto i = 0; i < nums.size(); i++) {
        int product = 1;
        for (auto j = i; j < nums.size(); j++) {
            product *= nums[j];
            if (finalProduct < product) {
                finalProduct = product;        
            }
        }
    }

    return finalProduct;
}

int main() {
    cout << "Hello, world! ~ C++" << endl;

    vector<int> nums = {-3,-1,-1};
    cout << "Anwser is =" << maxProduct(nums);

    return 0;
}
