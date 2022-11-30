/*
 * @Date: 2022-11-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-30
 * @FilePath: /algorithm/895_FreqStack/FreqStack.cpp
 */

#include <cassert>
#include <stack>
#include <unordered_map>

using namespace std;

class FreqStack {
 public:
  FreqStack() { maxFreq = 0; }

  void push(int val) {
    freq[val]++;
    group[freq[val]].push(val);
    maxFreq = max(maxFreq, freq[val]);
  }

  int pop() {
    int val = group[maxFreq].top();
    freq[val]--;
    group[maxFreq].pop();
    if (group[maxFreq].empty()) {
      maxFreq--;
    }
    return val;
  }

 private:
  unordered_map<int, int> freq;
  unordered_map<int, stack<int>> group;
  int maxFreq;
};

int main() {
  FreqStack f{};
  f.push(5);
  f.push(7);
  f.push(5);
  f.push(7);
  f.push(4);
  f.push(5);
  assert(f.pop() == 5);
  assert(f.pop() == 7);
  assert(f.pop() == 5);
  assert(f.pop() == 4);
}