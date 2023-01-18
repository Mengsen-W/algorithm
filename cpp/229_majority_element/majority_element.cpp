/*
 * @Date: 2021-10-22 00:48:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-22 01:01:18
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> majorityElement(vector<int>& nums) {
    int n = nums.size();
    vector<int> ans;
    unordered_map<int, int> cnt;

    for (auto& v : nums) cnt[v]++;

    for (auto& v : cnt)
      if (v.second > n / 3) ans.push_back(v.first);

    return ans;
  }
};

int main() {
  {
    vector<int> nums{3, 2, 3};
    assert(Solution().majorityElement(nums) == move(vector<int>{3}));
  }
  {
    vector<int> nums{1};
    assert(Solution().majorityElement(nums) == move(vector<int>{1}));
  }
  {
    vector<int> nums{1, 1, 1, 3, 3, 2, 2, 2};
    assert(Solution().majorityElement(nums) == move(vector<int>{2, 1}));
  }
}
