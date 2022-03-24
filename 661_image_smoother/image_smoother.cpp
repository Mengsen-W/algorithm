/*
 * @Date: 2022-03-23 23:29:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-23 23:34:46
 * @FilePath: /algorithm/661_image_smoother/image_smoother.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> imageSmoother(vector<vector<int>> img) {
    int m = img.size(), n = img[0].size();
    vector<vector<int>> ret(m, vector<int>(n));
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        int num = 0, sum = 0;
        for (int x = i - 1; x <= i + 1; x++) {
          for (int y = j - 1; y <= j + 1; y++) {
            if (x >= 0 && x < m && y >= 0 && y < n) {
              num++;
              sum += img[x][y];
            }
          }
        }
        ret[i][j] = sum / num;
      }
    }
    return ret;
  }
};

int main() {
  assert((Solution().imageSmoother(
              vector<vector<int>>{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}}) ==
          vector<vector<int>>{{0, 0, 0}, {0, 0, 0}, {0, 0, 0}}));
  assert(
      (Solution().imageSmoother(vector<vector<int>>{
           {100, 200, 100}, {200, 50, 200}, {100, 200, 100}}) ==
       vector<vector<int>>{{137, 141, 137}, {141, 138, 141}, {137, 141, 137}}));
}