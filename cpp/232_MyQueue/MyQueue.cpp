/*
 * @Date: 2024-03-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-04
 * @FilePath: /algorithm/cpp/232_MyQueue/MyQueue.cpp
 */

#include <cassert>
#include <stack>

using namespace std;

class MyQueue {
 private:
  stack<int> inStack, outStack;

  void in2out() {
    while (!inStack.empty()) {
      outStack.push(inStack.top());
      inStack.pop();
    }
  }

 public:
  MyQueue() {}

  void push(int x) { inStack.push(x); }

  int pop() {
    if (outStack.empty()) {
      in2out();
    }
    int x = outStack.top();
    outStack.pop();
    return x;
  }

  int peek() {
    if (outStack.empty()) {
      in2out();
    }
    return outStack.top();
  }

  bool empty() { return inStack.empty() && outStack.empty(); }
};

int main() {
  MyQueue myQueue = MyQueue();
  myQueue.push(1);                   // queue is: [1]
  myQueue.push(2);                   // queue is: [1, 2] (leftmost is front of the queue)
  assert(myQueue.peek() == 1);       // return 1
  assert(myQueue.pop() == 1);        // return 1, queue is [2]
  assert(myQueue.empty() == false);  // return false
}