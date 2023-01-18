/*
 * @Date: 2021-03-24 08:15:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-24 08:23:30
 * @FilePath: \algorithm\456_132_pattern\132_pattern.cpp
 * @Description: file content
 */

#include <cassert>
#include <stack>
#include <vector>
using namespace std;

bool find132_pattern(vector<int> nums) {
  int two = INT_MIN;

  stack<int> s;
  int n = nums.size();
  for (int i = n - 1; i >= 0; --i) {
    // 找到比2小的数字，那么满足条件
    if (nums[i] >= two) {
      // 如果发现大于最大栈，那么要去更新栈和two的数值（two变为更大的数字）
      while (!s.empty() && nums[s.top()] < nums[i]) {
        two = nums[s.top()];
        s.pop();
      }

      s.push(i);
    } else {
      return true;
    }
  }

  return false;
}

int main() {
  assert(!find132_pattern(vector<int>{1, 2, 3, 4}));
  assert(find132_pattern(vector<int>{3, 1, 4, 2}));
  assert(find132_pattern(vector<int>{-1, 3, 2, 0}));
  return 0;
}