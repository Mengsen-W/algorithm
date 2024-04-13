/*
 * @Date: 2024-04-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-13
 * @FilePath: /algorithm/cpp/2924_find_champion/find_champion.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findChampion(int n, vector<vector<int>>& edges) {
    vector<int> degree(n);
    for (auto e : edges) {
      degree[e[1]]++;
    }
    int champion = -1;
    for (int i = 0; i < n; i++) {
      if (degree[i] == 0) {
        if (champion == -1) {
          champion = i;
        } else {
          return -1;
        }
      }
    }
    return champion;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int>> tests{
      {3, {{0, 1}, {1, 2}}, 0},
      {4, {{0, 2}, {1, 3}, {1, 2}}, -1},
  };

  for (auto& [n, edges, ans] : tests) {
    assert(Solution().findChampion(n, edges) == ans);
  }
}