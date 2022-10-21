/*
 * @Date: 2022-10-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-21
 * @FilePath: /algorithm/901_StockSpanner/StockSpanner.cpp
 */

#include <cassert>
#include <stack>

using namespace std;

class StockSpanner {
 public:
  StockSpanner() {
    this->stk.emplace(-1, INT_MAX);
    this->idx = -1;
  }

  int next(int price) {
    idx++;
    while (price >= stk.top().second) {
      stk.pop();
    }
    int ret = idx - stk.top().first;
    stk.emplace(idx, price);
    return ret;
  }

 private:
  stack<pair<int, int>> stk;
  int idx;
};

int main() {
  StockSpanner s{};
  assert(s.next(100) == 1);
  assert(s.next(80) == 1);
  assert(s.next(60) == 1);
  assert(s.next(70) == 2);
  assert(s.next(60) == 1);
  assert(s.next(75) == 4);
  assert(s.next(85) == 6);
}