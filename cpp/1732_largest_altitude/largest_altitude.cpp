/*
 * @Date: 2022-11-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-19
 * @FilePath: /algorithm/1732_largest_altitude/largest_altitude.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int largestAltitude(vector<int>& gain) {
    int ans = 0, sum = 0;
    for (int x : gain) {
      sum += x;
      ans = max(ans, sum);
    }
    return ans;
  }
};

int main() {
  {
    vector<int> gain{-5, 1, 5, 0, -7};
    int ans = 1;
    assert(Solution().largestAltitude(gain) == ans);
  }

  {
    vector<int> gain{-4, -3, -2, -1, 4, 3, 2};
    int ans = 0;
    assert(Solution().largestAltitude(gain) == ans);
  }
}