#include <bits/stdc++.h>

using namespace std;

int strStr(string haystack, string needle) {
    if (needle.length() > haystack.length()) {
        return -1;
    }

    int window = needle.length() - 1;
    for (int i = 0; i < haystack.length(); i++) {
        for (int j = i, k = 0; k < needle.length(); j++, k++) {
            if (needle[k] == haystack[j]) {
                if (k == needle.length() - 1) {
                    return i;
                }
            } else {
                break;
            }
        }
    }

    return -1;
}

int main() {
    string haystack = "leetcode";
    string needle = "leeto";
    cout << "Found at Index = " << strStr(haystack, needle);

    return 0;
}
