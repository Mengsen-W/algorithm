/*
 * @Date: 2023-05-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-29
 * @FilePath: /algorithm/cpp/2455_average_value/average_value.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int averageValue(vector<int>& nums) {
    int total = 0, k = 0;
    for (int a : nums) {
      if (a % 6 == 0) {
        total += a;
        k++;
      }
    }
    return k > 0 ? total / k : 0;
  }
};

int main() {
  {
    vector<int> nums{1, 3, 6, 10, 12, 15};
    int ans = 9;
    assert(Solution().averageValue(nums) == ans);
  }

  {
    vector<int> nums{1, 2, 4, 7, 10};
    int ans = 0;
    assert(Solution().averageValue(nums) == ans);
  }
}