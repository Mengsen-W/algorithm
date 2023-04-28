/*
 * @Date: 2023-04-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-28
 * @FilePath: /algorithm/cpp/1172_DinnerPlates/DinnerPlates.cpp
 */

#include <cassert>
#include <set>
#include <vector>

using namespace std;

class DinnerPlates {
 public:
  DinnerPlates(int capacity) { this->capacity = capacity; }

  void push(int val) {
    if (poppedPos.empty()) {
      int pos = stk.size();
      stk.emplace_back(val);
      if (pos % capacity == 0) {
        top.emplace_back(0);
      } else {
        top.back()++;
      }
    } else {
      int pos = *poppedPos.begin();
      poppedPos.erase(pos);
      stk[pos] = val;
      int index = pos / capacity;
      top[index]++;
    }
  }

  int pop() {
    while (!stk.empty() && poppedPos.count(stk.size() - 1)) {
      stk.pop_back();
      int pos = *poppedPos.rbegin();
      poppedPos.erase(pos);
      if (pos % capacity == 0) {
        top.pop_back();
      }
    }
    if (stk.empty()) {
      return -1;
    } else {
      int pos = stk.size() - 1;
      int val = stk.back();
      stk.pop_back();
      if (pos % capacity == 0) {
        top.pop_back();
      } else {
        top.back() = top.size() - 2;
      }
      return val;
    }
  }

  int popAtStack(int index) {
    if (index >= top.size()) {
      return -1;
    }
    int stackTop = top[index];
    if (stackTop < 0) {
      return -1;
    }
    top[index]--;
    int pos = index * capacity + stackTop;
    poppedPos.emplace(pos);
    return stk[pos];
  }

 private:
  int capacity;
  vector<int> stk;
  vector<int> top;
  set<int> poppedPos;
};

int main() {
  DinnerPlates d = DinnerPlates(2);
  d.push(1);
  d.push(2);
  d.push(3);
  d.push(4);
  d.push(5);
  assert(d.popAtStack(0) == 2);
  d.push(20);
  d.push(21);
  assert(d.popAtStack(0) == 20);
  assert(d.popAtStack(2) == 21);
  assert(d.pop() == 5);
  assert(d.pop() == 4);
  assert(d.pop() == 3);
  assert(d.pop() == 1);
  assert(d.pop() == -1);
}