/*
 * @Date: 2023-09-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-19
 * @FilePath: /algorithm/cpp/2560_min_capability/min_capability.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minCapability(vector<int>& nums, int k) {
    int lower = *min_element(nums.begin(), nums.end());
    int upper = *max_element(nums.begin(), nums.end());
    while (lower <= upper) {
      int middle = (lower + upper) / 2;
      int count = 0;
      bool visited = false;
      for (int x : nums) {
        if (x <= middle && !visited) {
          count++;
          visited = true;
        } else {
          visited = false;
        }
      }
      if (count >= k) {
        upper = middle - 1;
      } else {
        lower = middle + 1;
      }
    }
    return lower;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{2, 3, 5, 9}, 2, 5},
      {{2, 7, 9, 3, 1}, 2, 2},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().minCapability(nums, k) == ans);
  }
}