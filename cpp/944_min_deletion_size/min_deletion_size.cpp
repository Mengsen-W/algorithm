/*
 * @Date: 2022-05-12 09:30:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-12 09:40:15
 * @FilePath: /algorithm/944_min_deletion_size/min_deletion_size.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int minDeletionSize(vector<string> strs) {
    int row = strs.size();
    int col = strs[0].size();
    int ans = 0;
    for (int j = 0; j < col; ++j) {
      for (int i = 1; i < row; ++i) {
        if (strs[i - 1][j] > strs[i][j]) {
          ans++;
          break;
        }
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().minDeletionSize({"cba", "daf", "ghi"}) == 1);
  assert(Solution().minDeletionSize({"a", "b"}) == 0);
  assert(Solution().minDeletionSize({"zyx", "wvu", "tsr"}) == 3);
}