/*
 * @Date: 2021-12-03 08:38:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-03 09:39:01
 */

#include <cassert>
#include <iostream>
#include <numeric>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int largestSumAfterKNegations(vector<int> nums, int k) {
    unordered_map<int, int> freq;
    for (int num : nums) freq[num] += 1;

    int ans = accumulate(nums.begin(), nums.end(), 0);
    for (int i = -100; i < 0; ++i)
      if (freq[i]) {
        int ops = min(k, freq[i]);
        ans += (-i) * ops * 2;
        freq[i] -= ops;
        freq[-i] += ops;
        k -= ops;
        if (k == 0) break;
      }

    if (k > 0 && k % 2 == 1 && !freq[0])
      for (int i = 1; i <= 100; ++i)
        if (freq[i]) {
          ans -= i * 2;
          break;
        }

    return ans;
  }
};

int main() {
  assert(Solution().largestSumAfterKNegations(vector<int>{4, 2, 3}, 1) == 5);
  assert(Solution().largestSumAfterKNegations(vector<int>{3, -1, 0, 2}, 3) ==
         6);
  assert(Solution().largestSumAfterKNegations(vector<int>{2, -3, -1, 5, -4},
                                              2) == 13);
  assert(Solution().largestSumAfterKNegations(
             vector<int>{-8, 3, -5, -3, -5, -2}, 6) == 22);
}