/*
 * @Date: 2021-08-26 09:55:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-26 11:05:31
 */

#include <cassert>
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
  {
    vector<int> nums{2, 7, 11, 15};
    int target = 9;
    vector<int> ans{0, 1};
    assert(Solution{}.twoSum(nums, target) == ans);
  }
  {
    vector<int> nums{3, 2, 4};
    int target = 6;
    vector<int> ans{1, 2};
    assert(Solution{}.twoSum(nums, target) == ans);
  }
  {
    vector<int> nums{3, 3};
    int target = 6;
    vector<int> ans{0, 1};
    assert(Solution{}.twoSum(nums, target) == ans);
  }
}