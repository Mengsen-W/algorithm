/*
 * @Date: 2023-11-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-28
 * @FilePath: /algorithm/cpp/1670_FrontMiddleBackQueue/FrontMiddleBackQueue.cpp
 */

#include <cassert>
#include <deque>

using namespace std;

class FrontMiddleBackQueue {
 public:
  FrontMiddleBackQueue() {}

  void pushFront(int val) {
    left.push_front(val);
    if (left.size() == right.size() + 2) {
      right.push_front(left.back());
      left.pop_back();
    }
  }

  void pushMiddle(int val) {
    if (left.size() == right.size() + 1) {
      right.push_front(left.back());
      left.pop_back();
    }
    left.push_back(val);
  }

  void pushBack(int val) {
    right.push_back(val);
    if (left.size() + 1 == right.size()) {
      left.push_back(right.front());
      right.pop_front();
    }
  }

  int popFront() {
    if (left.empty()) {
      return -1;
    }
    int val = left.front();
    left.pop_front();
    if (left.size() + 1 == right.size()) {
      left.push_back(right.front());
      right.pop_front();
    }
    return val;
  }

  int popMiddle() {
    if (left.empty()) {
      return -1;
    }
    int val = left.back();
    left.pop_back();
    if (left.size() + 1 == right.size()) {
      left.push_back(right.front());
      right.pop_front();
    }
    return val;
  }

  int popBack() {
    if (left.empty()) {
      return -1;
    }
    int val = 0;
    if (right.empty()) {
      val = left.back();
      left.pop_back();
    } else {
      val = right.back();
      right.pop_back();
      if (left.size() == right.size() + 2) {
        right.push_front(left.back());
        left.pop_back();
      }
    }
    return val;
  }

 private:
  deque<int> left;
  deque<int> right;
};

int main() {
  FrontMiddleBackQueue q = FrontMiddleBackQueue();
  q.pushFront(1);              // [1]
  q.pushBack(2);               // [1, 2]
  q.pushMiddle(3);             // [1, 3, 2]
  q.pushMiddle(4);             // [1, 4, 3, 2]
  assert(q.popFront() == 1);   // 返回 1 -> [4, 3, 2]
  assert(q.popMiddle() == 3);  // 返回 3 -> [4, 2]
  assert(q.popMiddle() == 4);  // 返回 4 -> [2]
  assert(q.popBack() == 2);    // 返回 2 -> []
  assert(q.popFront() == -1);  // 返回 -1 -> [] （队列为空）
}