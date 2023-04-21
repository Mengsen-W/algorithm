/*
 * @Date: 2023-04-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-21
 * @FilePath: /algorithm/cpp/2413_smallest_even_multiple/smallest_even_multiple.cpp
 */

#include <cassert>

using namespace std;

class Solution {
 public:
  int smallestEvenMultiple(int n) { return n % 2 == 0 ? n : 2 * n; }
};

int main() {
  assert(Solution().smallestEvenMultiple(5) == 10);
  assert(Solution().smallestEvenMultiple(6) == 6);
}