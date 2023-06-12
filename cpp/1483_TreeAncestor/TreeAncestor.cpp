/*
 * @Date: 2023-06-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-12
 * @FilePath: /algorithm/cpp/1483_TreeAncestor/TreeAncestor.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class TreeAncestor {
 public:
  constexpr static int Log = 16;
  vector<vector<int>> ancestors;

  TreeAncestor(int n, vector<int>& parent) {
    ancestors = vector<vector<int>>(n, vector<int>(Log, -1));
    for (int i = 0; i < n; i++) {
      ancestors[i][0] = parent[i];
    }
    for (int j = 1; j < Log; j++) {
      for (int i = 0; i < n; i++) {
        if (ancestors[i][j - 1] != -1) {
          ancestors[i][j] = ancestors[ancestors[i][j - 1]][j - 1];
        }
      }
    }
  }

  int getKthAncestor(int node, int k) {
    for (int j = 0; j < Log; j++) {
      if ((k >> j) & 1) {
        node = ancestors[node][j];
        if (node == -1) {
          return -1;
        }
      }
    }
    return node;
  }
};

int main() {
  vector<int> parent{-1, 0, 0, 1, 1, 2, 2};
  TreeAncestor t{7, parent};
  assert(t.getKthAncestor(3, 1) == 1);
  assert(t.getKthAncestor(5, 2) == 0);
  assert(t.getKthAncestor(6, 3) == -1);
}