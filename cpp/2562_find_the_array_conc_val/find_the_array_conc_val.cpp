/*
 * @Date: 2023-10-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-12
 * @FilePath: /algorithm/cpp/2562_find_the_array_conc_val/find_the_array_conc_val.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long findTheArrayConcVal(vector<int>& nums) {
    long long ans = 0;
    for (int i = 0, j = nums.size() - 1; i <= j; i++, j--) {
      if (i != j) {
        ans += stoi(to_string(nums[i]) + to_string(nums[j]));
      } else {
        ans += nums[i];
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{7, 52, 2, 4}, 596},
      {{5, 14, 13, 8, 12}, 673},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().findTheArrayConcVal(nums) == ans);
  }
}