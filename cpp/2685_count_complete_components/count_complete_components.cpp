#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class DSU {
 public:
  DSU(int n) : parent(n), rank(n, 1) {
    for (int i = 0; i < n; i++) {
      parent[i] = i;
    }
  }

  int find(int x) {
    if (parent[x] != x) {
      parent[x] = find(parent[x]);
    }
    return parent[x];
  }

  void unionSet(int x, int y) {
    x = find(x);
    y = find(y);
    if (x == y) {
      return;
    }
    if (rank[x] > rank[y]) {
      parent[y] = x;
    } else if (rank[y] > rank[x]) {
      parent[x] = y;
    } else {
      parent[x] = y;
      rank[y]++;
    }
  }

 private:
  vector<int> parent;
  vector<int> rank;
};

class Solution {
 public:
  int countCompleteComponents(int n, vector<vector<int>>& edges) {
    DSU dsu(n);
    for (auto& edge : edges) {
      dsu.unionSet(edge[0], edge[1]);
    }

    vector<int> numV(n, 0);
    vector<int> numE(n, 0);
    for (int i = 0; i < n; i++) {
      numV[dsu.find(i)]++;
    }
    for (auto& edge : edges) {
      numE[dsu.find(edge[0])]++;
    }

    int ans = 0;
    for (int i = 0; i < n; i++) {
      if (dsu.find(i) == i) {
        ans += numE[i] == (numV[i] * (numV[i] - 1) / 2);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int>> tests{
    {6, {{0, 1}, {0, 2}, {1, 2}, {3, 4}}, 3},
    {6, {{0, 1}, {0, 2}, {1, 2}, {3, 4}, {3, 5}}, 1},
  };

  for (auto& [n, edges, ans]: tests) {
    assert(Solution().countCompleteComponents(n, edges) == ans);
  }
  return 0;
}
