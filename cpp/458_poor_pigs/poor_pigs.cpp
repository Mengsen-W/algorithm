/*
 * @Date: 2021-11-25 02:40:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-25 02:43:43
 */

#include <cassert>
#include <cmath>

using namespace std;

class Solution {
 public:
  int poorPigs(int buckets, int minutesToDie, int minutesToTest) {
    int states = minutesToTest / minutesToDie + 1;
    int pigs = ceil(log(buckets) / log(states));
    return pigs;
  }
};

int main() {
  assert(Solution().poorPigs(1000, 15, 60) == 5);
  assert(Solution().poorPigs(4, 15, 15) == 2);
  assert(Solution().poorPigs(4, 15, 30) == 2);
}
