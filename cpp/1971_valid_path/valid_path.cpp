/*
 * @Date: 2022-12-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-19
 * @FilePath: /algorithm/1971_valid_path/valid_path.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class UnionFind {
 public:
  UnionFind(int n) {
    parent = vector<int>(n);
    rank = vector<int>(n);
    for (int i = 0; i < n; i++) {
      parent[i] = i;
    }
  }

  void uni(int x, int y) {
    int rootx = find(x);
    int rooty = find(y);
    if (rootx != rooty) {
      if (rank[rootx] > rank[rooty]) {
        parent[rooty] = rootx;
      } else if (rank[rootx] < rank[rooty]) {
        parent[rootx] = rooty;
      } else {
        parent[rooty] = rootx;
        rank[rootx]++;
      }
    }
  }

  int find(int x) {
    if (parent[x] != x) {
      parent[x] = find(parent[x]);
    }
    return parent[x];
  }

  bool connect(int x, int y) { return find(x) == find(y); }

 private:
  vector<int> parent;
  vector<int> rank;
};

class Solution {
 public:
  bool validPath(int n, vector<vector<int>>& edges, int source, int destination) {
    if (source == destination) {
      return true;
    }
    UnionFind uf(n);
    for (auto edge : edges) {
      uf.uni(edge[0], edge[1]);
    }
    return uf.connect(source, destination);
  }
};

int main() {
  {
    int n = 3;
    vector<vector<int>> edges{{0, 1}, {1, 2}, {2, 0}};
    int source = 0;
    int destination = 2;
    assert(Solution().validPath(n, edges, source, destination));
  }

  {
    int n = 6;
    vector<vector<int>> edges{{0, 1}, {0, 2}, {3, 5}, {5, 4}, {4, 3}};
    int source = 0;
    int destination = 5;
    assert(!Solution().validPath(n, edges, source, destination));
  }
}