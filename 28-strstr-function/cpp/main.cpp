#include <bits/stdc++.h>

using namespace std;

int strStr(string haystack, string needle) {
    if (needle.length() < 1 ) {
        return 0;
    }
    if (haystack.length() < 1 ) {
        return -1;
    }
    if (needle.length() > haystack.length()) {
        return -1;
    }

    for (int i = 0; i < haystack.length() - needle.length(); i++) {
        for (int j = 0; j < needle.length(); j++) {
            if (needle[j] != haystack[i + j]) {
                break;
            }
            if (j == needle.length() - 1) {
                return i;
            }
        }
    }

    return -1;
}

int main() {
    string haystack = "codeleetcode";
    string needle = "leet";
    cout << "Found at Index = " << strStr(haystack, needle);

    return 0;
}
