/*
 * @Date: 2021-08-26 09:55:37
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-01
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> twoSum(vector<int>& nums, int target) {
    unordered_map<int, int> hashtable;
    for (int i = 0; i < int(nums.size()); ++i) {
      auto it = hashtable.find(target - nums[i]);
      if (it != hashtable.end()) {
        return {it->second, i};
      }
      hashtable[nums[i]] = i;
    }
    return {};
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<int>>> testMap{
      {vector<int>{2, 7, 11, 15}, 9, vector<int>{0, 1}},
      {vector<int>{3, 2, 4}, 6, vector<int>{1, 2}},
      {vector<int>{3, 3}, 6, vector<int>{0, 1}},
  };

  for (auto& [nums, target, ans] : testMap) {
    assert(Solution().twoSum(nums, target) == ans);
  }
}