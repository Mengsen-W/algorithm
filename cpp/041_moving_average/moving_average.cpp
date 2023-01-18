/*
 * @Date: 2022-07-16
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-16
 * @FilePath: /algorithm/041_moving_average/moving_average.cpp
 */

#include <cassert>
#include <queue>

using namespace std;

class MovingAverage {
 public:
  MovingAverage(int size) {
    this->size = size;
    this->sum = 0.0;
  }

  double next(int val) {
    if (qu.size() == size) {
      sum -= qu.front();
      qu.pop();
    }
    qu.emplace(val);
    sum += val;
    return sum / qu.size();
  }

 private:
  int size;
  double sum;
  queue<int> qu;
};

int main() {
  MovingAverage m{3};
  assert(m.next(1) == 1.0);
  assert(m.next(10) == 5.5);
  assert(m.next(3) == 4.666666666666667);
  assert(m.next(5) == 6);
}