/*
 * @Date: 2023-06-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-14
 * @FilePath: /algorithm/cpp/1375_num_times_all_blue/num_times_all_blue.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int numTimesAllBlue(vector<int>& flips) {
    int n = flips.size();
    int ans = 0, right = 0;
    for (int i = 0; i < n; ++i) {
      right = max(right, flips[i]);
      if (right == i + 1) {
        ++ans;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> flips = {3, 2, 4, 1, 5};
    int ans = 2;
    assert(Solution().numTimesAllBlue(flips) == ans);
  }

  {
    vector<int> flips = {4, 1, 2, 3};
    int ans = 1;
    assert(Solution().numTimesAllBlue(flips) == ans);
  }
}