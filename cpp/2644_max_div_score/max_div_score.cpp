/*
 * @Date: 2024-05-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-18
 * @FilePath: /algorithm/cpp/2644_max_div_score/max_div_score.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxDivScore(vector<int>& nums, vector<int>& divisors) {
    int cnt = -1, ans = 0;

    for (int i = 0; i < divisors.size(); i++) {
      int tmp = 0;
      for (int j = 0; j < nums.size(); j++) {
        if (nums[j] % divisors[i] == 0) {
          tmp++;
        }
      }

      if (tmp > cnt || (tmp == cnt && divisors[i] < ans)) {
        ans = divisors[i];
        cnt = tmp;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{4, 7, 9, 3, 9}, {5, 2, 3}, 3},
      {{20, 14, 21, 10}, {5, 7, 5}, 5},
      {{12}, {10, 16}, 10},
  };

  for (auto& [nums, divisors, ans] : tests) {
    assert(Solution().maxDivScore(nums, divisors) == ans);
  }
}