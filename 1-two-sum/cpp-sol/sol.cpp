#include "iostream"
#include "vector"
#include <map>
using namespace std;

int main () {
    vector<int> nums, sol;
    map<int, int> mem;
    int target;

    nums = {3, 2, 4};
    target = 6;

    int j = 0;
    mem[nums[0]] = j;
    for (auto i = ++nums.begin(); i != nums.end(); i++, j++) {
        if (mem.find(target - *i) == mem.end()) {
            continue;
        } else {
            cout << "target found at= " << j << "value of target=" << *i << endl;
            sol.push_back(mem[target - *i]);
            sol.push_back(j);
            break;
        }
    }

    cout << "0=" << sol[0] << endl;
    cout << "1=" << sol[1] << endl;

    return 0;
}
