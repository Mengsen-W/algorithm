/*
 * @Date: 2022-04-18 07:14:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-18 07:24:30
 * @FilePath: /algorithm/386_lexical_order/lexical_order.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> lexicalOrder(int n) {
    vector<int> ret(n);
    int number = 1;
    for (int i = 0; i < n; i++) {
      ret[i] = number;
      if (number * 10 <= n) {
        number *= 10;
      } else {
        while (number % 10 == 9 || number + 1 > n) {
          number /= 10;
        }
        number++;
      }
    }
    return ret;
  }
};

int main() {
  assert(Solution().lexicalOrder(13) == vector<int>({1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9}));
  assert(Solution().lexicalOrder(2) == vector<int>({1, 2}));
}