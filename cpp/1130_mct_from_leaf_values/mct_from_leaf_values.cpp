/*
 * @Date: 2023-05-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-31
 * @FilePath: /algorithm/cpp/1130_mct_from_leaf_values/mct_from_leaf_values.cpp
 */

#include <cassert>
#include <stack>
#include <vector>

using namespace std;

class Solution {
 public:
  int mctFromLeafValues(vector<int>& arr) {
    int res = 0;
    stack<int> stk;
    for (int x : arr) {
      while (!stk.empty() && stk.top() <= x) {
        int y = stk.top();
        stk.pop();
        if (stk.empty() || stk.top() > x) {
          res += y * x;
        } else {
          res += stk.top() * y;
        }
      }
      stk.push(x);
    }
    while (stk.size() >= 2) {
      int x = stk.top();
      stk.pop();
      res += stk.top() * x;
    }
    return res;
  }
};

int main() {
  {
    vector<int> arr = {6, 2, 4};
    int ans = 32;
    assert(Solution().mctFromLeafValues(arr) == ans);
  }

  {
    vector<int> arr = {4, 11};
    int ans = 44;
    assert(Solution().mctFromLeafValues(arr) == ans);
  }
}
