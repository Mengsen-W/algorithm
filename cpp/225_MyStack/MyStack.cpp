/*
 * @Date: 2024-03-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-03
 * @FilePath: /algorithm/cpp/225_MyStack/MyStack.cpp
 */

#include <cassert>
#include <queue>

using namespace std;

class MyStack {
 public:
  queue<int> q;

  /** Initialize your data structure here. */
  MyStack() {}

  /** Push element x onto stack. */
  void push(int x) {
    int n = q.size();
    q.push(x);
    for (int i = 0; i < n; i++) {
      q.push(q.front());
      q.pop();
    }
  }

  /** Removes the element on top of the stack and returns that element. */
  int pop() {
    int r = q.front();
    q.pop();
    return r;
  }

  /** Get the top element. */
  int top() {
    int r = q.front();
    return r;
  }

  /** Returns whether the stack is empty. */
  bool empty() { return q.empty(); }
};

int main() {
  MyStack myStack = MyStack();
  myStack.push(1);
  myStack.push(2);
  assert(myStack.top() == 2);  // 返回 2
  assert(myStack.pop() == 2);  // 返回 2
  assert(!myStack.empty());    // 返回 False
}