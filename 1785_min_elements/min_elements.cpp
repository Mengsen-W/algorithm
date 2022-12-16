/*
 * @Date: 2022-12-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-16
 * @FilePath: /algorithm/1785_min_elements/min_elements.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minElements(vector<int>& nums, int limit, int goal) {
    long long sum = 0;
    for (auto x : nums) {
      sum += x;
    }
    long long diff = abs(sum - goal);
    return (diff + limit - 1) / limit;
  }
};

int main() {
  {
    vector<int> nums{1, -1, 1};
    int limit = 3;
    int goal = -4;
    int ans = 2;
    assert(Solution().minElements(nums, limit, goal) == ans);
  }

  {
    vector<int> nums{1, -10, 9, 1};
    int limit = 100;
    int goal = 0;
    int ans = 1;
    assert(Solution().minElements(nums, limit, goal) == ans);
  }
}