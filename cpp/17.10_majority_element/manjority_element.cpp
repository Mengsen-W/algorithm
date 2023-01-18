/*
 * @Date: 2021-07-09 09:11:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-09 09:29:33
 */

#include <algorithm>
#include <cassert>
#include <vector>
using namespace std;

int majorityElement(vector<int>& nums) {
  pair<int, int> ret;
  for (int i = 0; i < nums.size(); i++) {
    if (ret.first == nums[i]) {
      ret.second++;
    } else if (ret.second == 0) {
      ret.first = nums[i];
      ret.second = 1;
    } else {
      ret.second--;
    }
  }
  return (count(nums.begin(), nums.end(), ret.first) > (nums.size() / 2))
             ? ret.first
             : -1;
}

int main() {
  {
    vector<int> nums{1, 2, 5, 9, 5, 9, 5, 5, 5};
    assert(majorityElement(nums) == 5);
  }
  {
    vector<int> nums{3, 2};
    assert(majorityElement(nums) == -1);
  }
  {
    vector<int> nums{2, 2, 1, 1, 1, 2, 2};
    assert(majorityElement(nums) == 2);
  }
}