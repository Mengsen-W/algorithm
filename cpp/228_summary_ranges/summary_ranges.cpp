/*
 * @Date: 2023-08-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-26
 * @FilePath: /algorithm/cpp/228_summary_ranges/summary_ranges.cpp
 */

#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> summaryRanges(vector<int>& nums) {
    vector<string> ans{};
    for (size_t left = 0; left < nums.size(); left++) {
      size_t right = left;
      int gap = 0;
      while (right < nums.size() && nums[right] == nums[left] + gap) {
        right++;
        gap++;
      }
      // 走过一个，调整回去
      right--;
      // std::cout << left << " " << right - 1 << std::endl;
      ans.emplace_back(to_string(nums[left]) + (right == left ? "" : "->" + to_string(nums[right])));
      left = right;
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<string>>> test_cases{
      {{1, 2, 3, 4, 5}, {"1->5"}},
      {{0, 1, 2, 4, 5, 7}, {"0->2", "4->5", "7"}},
      {{0, 2, 3, 4, 6, 8, 9}, {"0", "2->4", "6", "8->9"}},
  };

  for (auto& [nums, ans] : test_cases) {
    assert(Solution().summaryRanges(nums) == ans);
  }
}