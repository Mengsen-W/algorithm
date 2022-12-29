/*
 * @Date: 2022-12-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-29
 * @FilePath: /algorithm/2023_two_out_of_three/two_out_of_three.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> twoOutOfThree(vector<int>& nums1, vector<int>& nums2, vector<int>& nums3) {
    unordered_map<int, int> mp;
    for (auto& i : nums1) {
      mp[i] = 1;
    }
    for (auto& i : nums2) {
      mp[i] |= 2;
    }
    for (auto& i : nums3) {
      mp[i] |= 4;
    }
    vector<int> res;
    for (auto& [k, v] : mp) {
      if (v & (v - 1)) {
        res.push_back(k);
      }
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums1{1, 1, 3, 2};
    vector<int> nums2{2, 3};
    vector<int> nums3{3};
    vector<int> ans{2, 3};
    assert(Solution().twoOutOfThree(nums1, nums2, nums3) == ans);
  }

  {
    vector<int> nums1{3, 1};
    vector<int> nums2{2, 3};
    vector<int> nums3{1, 2};
    vector<int> ans{1, 2, 3};
    assert(Solution().twoOutOfThree(nums1, nums2, nums3) == ans);
  }

  {
    vector<int> nums1{1, 2, 2};
    vector<int> nums2{4, 3, 3};
    vector<int> nums3{5};
    vector<int> ans{};
    assert(Solution().twoOutOfThree(nums1, nums2, nums3) == ans);
  }
}