/*
 * @Date: 2021-11-10 00:52:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-10 01:02:24
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int findPoisonedDuration(vector<int> timeSeries, int duration) {
    int n = timeSeries.size();
    int sum = 0;
    int l = 0, r = 0;
    for (int i = 0; i < n; i++)
      if (timeSeries[i] > r) {
        sum += r - l;
        l = timeSeries[i];
        r = timeSeries[i] + duration;
      } else
        r = timeSeries[i] + duration;

    sum += r - l;
    return sum;
  }
};

int main() {
  assert(Solution().findPoisonedDuration({1, 4}, 2) == 4);
  assert(Solution().findPoisonedDuration({1, 2}, 2) == 3);
}