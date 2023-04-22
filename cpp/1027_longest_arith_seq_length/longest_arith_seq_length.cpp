/*
 * @Date: 2023-04-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-22
 * @FilePath: /algorithm/cpp/1027_longest_arith_seq_length/longest_arith_seq_length.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestArithSeqLength(vector<int>& nums) {
    auto [minit, maxit] = minmax_element(nums.begin(), nums.end());
    int diff = *maxit - *minit;
    int ans = 1;
    for (int d = -diff; d <= diff; ++d) {
      vector<int> f(*maxit + 1, -1);
      for (int num : nums) {
        if (int prev = num - d; prev >= *minit && prev <= *maxit && f[prev] != -1) {
          f[num] = max(f[num], f[prev] + 1);
          ans = max(ans, f[num]);
        }
        f[num] = max(f[num], 1);
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{3, 6, 9, 12};
    int ans = 4;
    assert(Solution().longestArithSeqLength(nums) == ans);
  }

  {
    vector<int> nums{9, 4, 7, 2, 10};
    int ans = 3;
    assert(Solution().longestArithSeqLength(nums) == ans);
  }

  {
    vector<int> nums{20, 1, 15, 3, 10, 5, 8};
    int ans = 4;
    assert(Solution().longestArithSeqLength(nums) == ans);
  }
}