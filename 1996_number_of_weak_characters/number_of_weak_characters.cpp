/*
 * @Date: 2022-01-28 01:23:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-28 01:39:51
 */

#include <cassert>
#include <stack>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfWeakCharacters(vector<vector<int>> properties) {
    sort(begin(properties), end(properties),
         [](const vector<int>& a, const vector<int>& b) {
           return a[0] == b[0] ? (a[1] > b[1]) : (a[0] < b[0]);
         });
    stack<int> st;
    int ans = 0;
    for (auto& p : properties) {
      while (!st.empty() && st.top() < p[1]) {
        ++ans;
        st.pop();
      }
      st.push(p[1]);
    }
    return ans;
  }
};

int main() {
  assert(Solution().numberOfWeakCharacters({{5, 5}, {6, 3}, {3, 6}}) == 0);
  assert(Solution().numberOfWeakCharacters({{2, 2}, {3, 3}}) == 1);
  assert(Solution().numberOfWeakCharacters({{1, 5}, {10, 4}, {4, 3}}) == 1);
}