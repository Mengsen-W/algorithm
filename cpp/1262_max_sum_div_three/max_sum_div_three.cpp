/*
 * @Date: 2023-06-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-19
 * @FilePath: /algorithm/cpp/1262_max_sum_div_three/max_sum_div_three.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxSumDivThree(vector<int>& nums) {
    vector<int> f = {0, INT_MIN, INT_MIN};
    for (int num : nums) {
      vector<int> g = f;
      for (int i = 0; i < 3; ++i) {
        g[(i + num % 3) % 3] = max(g[(i + num % 3) % 3], f[i] + num);
      }
      f = std::move(g);
    }
    return f[0];
  }
};

int main() {
  {
    vector<int> nums{3, 6, 5, 1, 8};
    int ans = 18;
    assert(Solution().maxSumDivThree(nums) == ans);
  }

  {
    vector<int> nums{4};
    int ans = 0;
    assert(Solution().maxSumDivThree(nums) == ans);
  }

  {
    vector<int> nums{1, 2, 3, 4, 4};
    int ans = 12;
    assert(Solution().maxSumDivThree(nums) == ans);
  }
}