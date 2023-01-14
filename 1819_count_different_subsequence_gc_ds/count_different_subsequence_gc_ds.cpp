/*
 * @Date: 2023-01-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-14
 * @FilePath: /algorithm/1819_count_different_subsequence_gc_ds/count_different_subsequence_gc_ds.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int countDifferentSubsequenceGCDs(vector<int>& nums) {
    int maxVal = *max_element(nums.begin(), nums.end());
    vector<bool> occured(maxVal + 1, false);
    for (int num : nums) {
      occured[num] = true;
    }
    int ans = 0;
    for (int i = 1; i <= maxVal; i++) {
      int subGcd = 0;
      for (int j = i; j <= maxVal; j += i) {
        if (occured[j]) {
          if (subGcd == 0) {
            subGcd = j;
          } else {
            subGcd = gcd(subGcd, j);
          }
          if (subGcd == i) {
            ans++;
            break;
          }
        }
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{6, 10, 3};
    int ans = 5;
    assert(Solution().countDifferentSubsequenceGCDs(nums) == ans);
  }

  {
    vector<int> nums{5, 15, 40, 5, 6};
    int ans = 7;
    assert(Solution().countDifferentSubsequenceGCDs(nums) == ans);
  }
}