/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-06 10:04:11
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-06 10:22:15
 */

#include <bits/stdc++.h>

using namespace std;

vector<int> next_greater_elements(vector<int>&& nums) {
  int n = nums.size();
  vector<int> ret(n, -1);
  stack<int> stk;
  for (int i = 0; i < n * 2 - 1; i++) {
    while (!stk.empty() && nums[stk.top()] < nums[i % n]) {
      ret[stk.top()] = nums[i % n];
      stk.pop();
    }
    stk.push(i % n);
  }
  return ret;
}

int main() {
  assert(next_greater_elements(vector<int>{1, 2, 1}) ==
         move(vector<int>{2, -1, 2}));
  return 0;
}