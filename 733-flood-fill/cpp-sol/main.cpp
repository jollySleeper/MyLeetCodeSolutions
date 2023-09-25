/**
 * Flood Fill
 * https://leetcode.com/problems/flood-fill/
 */

#include "iostream"
#include "vector"
using namespace std;

/**
 * Depth First Search Implementaion
 * O(1) for space
 * O(m*n) for time: as in worst case it will traverse the matrix
 */
vector<vector<int>> floodFillDFS(vector<vector<int>> &image, int sr, int sc,
                                 int color, int prevColor) {
    if (image[sr][sc] != color && image[sr][sc] == prevColor) {
        image[sr][sc] = color;
        if (sr > 0) {
            floodFillDFS(image, sr - 1, sc, color, prevColor);
        }
        if ((sr + 1) < image.size()) {
            floodFillDFS(image, sr + 1, sc, color, prevColor);
        }
        if (sc > 0) {
            floodFillDFS(image, sr, sc - 1, color, prevColor);
        }
        if ((sc + 1) < image[sr].size()) {
            floodFillDFS(image, sr, sc + 1, color, prevColor);
        }
    }

    return image;
}

/**
 * Bread First Search Implementaion
 */
vector<vector<int>> floodFillBFS(vector<vector<int>> &image, int sr, int sc,
                                 int color) {
    int oldColor = image[sr][sc];
    image[sr][sc] = color;
    vector<int> queue = {0};
    for (int i = 0; i < image.size(); i++) {
        for (int j = 0; j < image[i].size(); j++) {
            if (image[i][j] == oldColor) {
                queue.push_back(image[i][j]);
            }
        }
    }

    return image;
}

vector<vector<int>> floodFill(vector<vector<int>> &image, int sr, int sc,
                              int color) {
    // DFS Implementaion
    // return floodFillDFS(image, sr, sc, color, image[sr][sc]);
    // BFS Implementaion
    return floodFillBFS(image, sr, sc, color);
}

int main(int argc, char *argv[]) {
    cout << "hello world" << endl;

    vector<vector<int>> pixels = {{1, 1, 1}, {1, 1, 0}, {1, 0, 1}};
    for (int i = 0; i < pixels.size(); i++) {
        for (int j = 0; j < pixels[i].size(); j++) {
            cout << pixels[i][j];
        }
        cout << endl;
    }

    vector<vector<int>> paintedPixel = floodFill(pixels, 1, 1, 2);
    for (int i = 0; i < paintedPixel.size(); i++) {
        for (int j = 0; j < paintedPixel[i].size(); j++) {
            cout << paintedPixel[i][j];
        }
        cout << endl;
    }

    return 0;
}
