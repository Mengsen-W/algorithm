/*
 * @Date: 2021-12-19 01:00:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-19 01:16:37
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int findJudge(int n, vector<vector<int>> trust) {
    vector<int> inDegrees(n + 1);
    vector<int> outDegrees(n + 1);
    for (auto& edge : trust) {
      int x = edge[0], y = edge[1];
      ++inDegrees[y];
      ++outDegrees[x];
    }
    for (int i = 1; i <= n; ++i) {
      if (inDegrees[i] == n - 1 && outDegrees[i] == 0) return i;
    }
    return -1;
  }
};

int main() {
  assert(Solution().findJudge(2, {{1, 2}}) == 2);
  assert(Solution().findJudge(3, {{1, 3}, {2, 3}}) == 3);
  assert(Solution().findJudge(3, {{1, 3}, {2, 3}, {3, 1}}) == -1);
  assert(Solution().findJudge(3, {{1, 2}, {2, 3}}) == -1);
  assert(Solution().findJudge(4, {{1, 3}, {1, 4}, {2, 3}, {2, 4}, {4, 3}}) ==
         3);
}