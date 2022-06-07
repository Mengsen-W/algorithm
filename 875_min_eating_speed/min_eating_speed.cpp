/*
 * @Date: 2022-06-07 09:54:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-07 09:57:06
 * @FilePath: /algorithm/875_min_eating_speed/min_eating_speed.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minEatingSpeed(vector<int> piles, int h) {
    int low = 1;
    int high = 0;
    for (int pile : piles) {
      high = max(high, pile);
    }
    int k = high;
    while (low < high) {
      int speed = (high - low) / 2 + low;
      long time = getTime(piles, speed);
      if (time <= h) {
        k = speed;
        high = speed;
      } else {
        low = speed + 1;
      }
    }
    return k;
  }

  long getTime(const vector<int>& piles, int speed) {
    long time = 0;
    for (int pile : piles) {
      int curTime = (pile + speed - 1) / speed;
      time += curTime;
    }
    return time;
  }
};

int main() {
  assert(Solution().minEatingSpeed({3, 6, 7, 11}, 8) == 4);
  assert(Solution().minEatingSpeed({30, 11, 23, 4, 20}, 5) == 30);
  assert(Solution().minEatingSpeed({30, 11, 23, 4, 20}, 6) == 23);
}