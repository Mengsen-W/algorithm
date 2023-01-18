/*
 * @Date: 2022-06-13 09:52:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-13 10:04:51
 * @FilePath: /algorithm/1051_height_checker/height_checker.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int heightChecker(vector<int> heights) {
    int m = *max_element(heights.begin(), heights.end());
    vector<int> cnt(m + 1);
    for (int h : heights) {
      ++cnt[h];
    }

    int idx = 0, ans = 0;
    for (int i = 1; i <= m; ++i) {
      for (int j = 1; j <= cnt[i]; ++j) {
        if (heights[idx] != i) {
          ++ans;
        }
        ++idx;
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().heightChecker({1, 1, 4, 2, 1, 3}) == 3);
  assert(Solution().heightChecker({5, 1, 2, 3, 4}) == 5);
  assert(Solution().heightChecker({1, 2, 3, 4, 5}) == 0);
}
