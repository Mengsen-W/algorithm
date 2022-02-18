/*
 * @Date: 2022-02-18 02:02:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-18 02:09:51
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution1 {
 public:
  int findCenter(vector<vector<int>> edges) {
    int n = edges.size() + 1;
    vector<int> degrees(n + 1);
    for (auto& edge : edges) {
      degrees[edge[0]]++;
      degrees[edge[1]]++;
    }
    for (int i = 1; i <= n; i++) {
      if (degrees[i] == n - 1) {
        return i;
      }
    }
    return 0;
  }
};

class Solution2 {
 public:
  int findCenter(vector<vector<int>> edges) {
    return edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1]
               ? edges[0][0]
               : edges[0][1];
  }
};

int main() {
  assert(Solution1().findCenter(vector<vector<int>>{{1, 2}, {2, 3}, {4, 2}}) ==
         2);
  assert(Solution2().findCenter(vector<vector<int>>{{1, 2}, {2, 3}, {4, 2}}) ==
         2);
  assert(Solution1().findCenter(
             vector<vector<int>>{{1, 2}, {5, 1}, {1, 3}, {1, 4}}) == 1);
  assert(Solution2().findCenter(
             vector<vector<int>>{{1, 2}, {5, 1}, {1, 3}, {1, 4}}) == 1);
}