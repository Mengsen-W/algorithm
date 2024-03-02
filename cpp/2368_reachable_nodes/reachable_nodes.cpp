/*
 * @Date: 2024-03-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-02
 * @FilePath: /algorithm/cpp/2368_reachable_nodes/reachable_nodes.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class UnionFind {
 public:
  UnionFind(int n) : f(n), rank(n) {
    for (int i = 0; i < n; i++) {
      f[i] = i;
    }
  }

  void merge(int x, int y) {
    int rx = find(x);
    int ry = find(y);
    if (rx != ry) {
      if (rank[rx] > rank[ry]) {
        f[ry] = rx;
      } else if (rank[rx] < rank[ry]) {
        f[rx] = ry;
      } else {
        f[ry] = rx;
        rank[rx]++;
      }
    }
  }

  int find(int x) {
    if (x != f[x]) {
      x = find(f[x]);
    }
    return f[x];
  }

  int count() {
    int cnt = 0;
    int rt = find(0);
    for (int i = 0; i < f.size(); i++) {
      if (rt == find(i)) {
        cnt++;
      }
    }
    return cnt;
  }

 private:
  vector<int> f;
  vector<int> rank;
};
class Solution {
 public:
  int reachableNodes(int n, vector<vector<int>>& edges, vector<int>& restricted) {
    vector<int> isrestricted(n);
    for (auto& x : restricted) {
      isrestricted[x] = 1;
    }

    UnionFind uf = UnionFind(n);
    for (auto& v : edges) {
      if (isrestricted[v[0]] || isrestricted[v[1]]) {
        continue;
      }
      uf.merge(v[0], v[1]);
    }
    return uf.count();
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>, int>> tests{
      {7, {{0, 1}, {1, 2}, {3, 1}, {4, 0}, {0, 5}, {5, 6}}, {4, 5}, 4},
      {7, {{0, 1}, {0, 2}, {0, 5}, {0, 4}, {3, 2}, {6, 5}}, {4, 2, 1}, 3},
  };

  for (auto& [n, edges, restricted, ans] : tests) {
    assert(Solution().reachableNodes(n, edges, restricted) == ans);
  }
}