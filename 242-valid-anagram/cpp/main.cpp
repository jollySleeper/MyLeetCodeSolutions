#include <bits/stdc++.h>

using namespace std;

bool isAnagram(string s, string t) {
    if (s.size() != t.size()) {
        return false;
    }

    unordered_map<char, int> baseStringMap;
    for (auto chs:s) {
        baseStringMap[chs]++;
    }
    
    int counter = 0;
    for (auto cht:t) {
        baseStringMap[cht] -= 1;
        counter++;
        if (counter == s.size()) {
            for (auto it = baseStringMap.begin(); it != baseStringMap.end(); ++it) {
                if (it->second != 0) {
                    return false;
                }
            }
            return true;
        }
    }
    
    return false;
}

int main() {
    cout << "Hello, world! ~ C++" << endl;
    string base = "anagram";
    string test = "nagaram";
    cout << "Are Strings Anagram = " << isAnagram(base, test) << endl;

    return 0;
}
