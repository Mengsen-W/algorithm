/*
 * @Date: 2021-10-25 00:50:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-25 01:10:43
 */

#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  bool searchMatrix(vector<vector<int>>& matrix, int target) {
    int m = matrix.size(), n = matrix[0].size();
    // 从右上角开始查找
    int x = 0, y = n - 1;
    while (x < m && y >= 0) {
      // cout << "x = " << x << " y = " << y << endl;
      if (matrix[x][y] == target)
        return true;
      else if (matrix[x][y] < target)
        ++x;
      else if (matrix[x][y] > target)
        --y;
    }
    return false;
  }
};

int main() {
  {
    vector<vector<int>> matrix = {{1, 4, 7, 11, 15},
                                  {2, 5, 8, 12, 19},
                                  {3, 6, 9, 16, 22},
                                  {10, 13, 14, 17, 24},
                                  {18, 21, 23, 26, 30}};
    int target = 5;
    assert(Solution().searchMatrix(matrix, target));
  }
  {
    vector<vector<int>> matrix = {{1, 4, 7, 11, 15},
                                  {2, 5, 8, 12, 19},
                                  {3, 6, 9, 16, 22},
                                  {10, 13, 14, 17, 24},
                                  {18, 21, 23, 26, 30}};
    int target = 20;
    assert(!Solution().searchMatrix(matrix, target));
  }
  {
    vector<vector<int>> matrix = {{-5}};
    int target = -10;
    assert(!Solution().searchMatrix(matrix, target));
  }
}