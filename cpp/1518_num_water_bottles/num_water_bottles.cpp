/*
 * @Date: 2021-12-17 08:27:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-17 08:28:14
 */

#include <cassert>

class Solution {
 public:
  int numWaterBottles(int numBottles, int numExchange) {
    return numBottles >= numExchange
               ? (numBottles - numExchange) / (numExchange - 1) + 1 + numBottles
               : numBottles;
  }
};

int main() {
  assert(Solution().numWaterBottles(9, 3) == 13);
  assert(Solution().numWaterBottles(15, 4) == 19);
  assert(Solution().numWaterBottles(5, 5) == 6);
  assert(Solution().numWaterBottles(2, 3) == 2);
}