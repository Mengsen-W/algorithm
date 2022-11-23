/*
 * @Date: 2022-11-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-23
 * @FilePath: /algorithm/1742_count_balls/count_balls.cpp
 */

#include <cassert>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  int countBalls(int lowLimit, int highLimit) {
    unordered_map<int, int> count;
    int res = 0;
    for (int i = lowLimit; i <= highLimit; i++) {
      int box = 0, x = i;
      while (x) {
        box += x % 10;
        x /= 10;
      }
      count[box]++;
      res = max(res, count[box]);
    }
    return res;
  }
};

int main() {
  assert(Solution().countBalls(1, 10) == 2);
  assert(Solution().countBalls(5, 15) == 2);
  assert(Solution().countBalls(19, 28) == 2);
}