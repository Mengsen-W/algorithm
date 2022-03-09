/*
 * @Date: 2022-03-09 00:34:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-09 00:57:34
 * @FilePath: /algorithm/798_best_rotation/best_rotation.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int bestRotation(vector<int> nums) {
    int n = nums.size();
    vector<int> diffs(n);
    for (int i = 0; i < n; i++) {
      int low = (i + 1) % n;
      int high = (i - nums[i] + n + 1) % n;
      diffs[low]++;
      diffs[high]--;
      if (low >= high) {
        diffs[0]++;
      }
    }
    int bestIndex = 0;
    int maxScore = 0;
    int score = 0;
    for (int i = 0; i < n; i++) {
      score += diffs[i];
      if (score > maxScore) {
        bestIndex = i;
        maxScore = score;
      }
    }
    return bestIndex;
  }
};

int main() {
  assert(Solution().bestRotation({2, 3, 1, 4, 0}) == 3);
  assert(Solution().bestRotation({1, 3, 0, 2, 4}) == 0);
}