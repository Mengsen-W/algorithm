/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-05 10:10:29
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-05 10:14:05
 */

#include <cassert>
#include <stack>

using namespace std;

class MyQueue {
 public:
  /** Initialize your data structure here. */
  MyQueue() {}

  /** Push element x to the back of queue. */
  void push(int x) { in_stk.push(x); }

  /** Removes the element from in front of queue and returns that element. */
  int pop() {
    if (out_stk.empty()) in_to_out();
    int tmp = out_stk.top();
    out_stk.pop();
    return tmp;
  }

  /** Get the front element. */
  int peek() {
    if (out_stk.empty()) in_to_out();
    return out_stk.top();
  }

  /** Returns whether the queue is empty. */
  bool empty() { return in_stk.empty() && out_stk.empty(); }

 private:
  void in_to_out() {
    while (!in_stk.empty()) {
      out_stk.push(in_stk.top());
      in_stk.pop();
    }
  }

 private:
  stack<int> in_stk;
  stack<int> out_stk;
};

int main(void) {
  MyQueue* obj = new MyQueue();
  obj->push(1);
  obj->push(2);
  assert(1 == obj->peek());
  assert(1 == obj->pop());
  assert(!obj->empty());

  return 0;
}