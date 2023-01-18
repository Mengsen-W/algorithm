/*
 * @Date: 2021-08-14 13:51:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-14 14:15:52
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int unhappyFriends(int n, vector<vector<int>>& preferences,
                     vector<vector<int>>& pairs) {
    vector<vector<int>> order(n, vector<int>(n));
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n - 1; ++j) {
        order[i][preferences[i][j]] = j;
      }
    }
    vector<int> match(n);
    for (const auto& pr : pairs) {
      match[pr[0]] = pr[1];
      match[pr[1]] = pr[0];
    }

    int unhappyCount = 0;
    for (int x = 0; x < n; ++x) {
      int index = order[x][match[x]];
      for (int i = 0; i < index; ++i) {
        int u = preferences[x][i];
        int v = match[u];
        if (order[u][x] < order[u][v]) {
          ++unhappyCount;
          break;
        }
      }
    }
    return unhappyCount;
  }
};

int main() {
  {
    int n = 4;
    vector<vector<int>> preferences{{1, 2, 3}, {3, 2, 0}, {3, 1, 0}, {1, 2, 0}};
    vector<vector<int>> pairs{{0, 1}, {2, 3}};
    assert(Solution{}.unhappyFriends(n, preferences, pairs) == 2);
  }

  {
    int n = 2;
    vector<vector<int>> preferences{{1}, {0}};
    vector<vector<int>> pairs{{1, 0}};
    assert(Solution{}.unhappyFriends(n, preferences, pairs) == 0);
  }

  {
    int n = 4;
    vector<vector<int>> preferences{{1, 3, 2}, {2, 3, 0}, {1, 3, 0}, {0, 2, 1}};
    vector<vector<int>> pairs{{1, 3}, {0, 2}};
    assert(Solution{}.unhappyFriends(n, preferences, pairs) == 4);
  }

  return 0;
}