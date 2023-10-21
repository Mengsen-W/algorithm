/*
 * @Date: 2023-10-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-21
 * @FilePath: /algorithm/cpp/2316_count_pairs/count_pairs.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class UnionFind {
 private:
  vector<int> parents;
  vector<int> sizes;

 public:
  UnionFind(int n) : parents(n), sizes(n, 1) { iota(parents.begin(), parents.end(), 0); }
  int Find(int x) {
    if (parents[x] == x) {
      return x;
    }
    return parents[x] = Find(parents[x]);
  }
  void Union(int x, int y) {
    int rx = Find(x), ry = Find(y);
    if (rx != ry) {
      if (sizes[rx] > sizes[ry]) {
        parents[ry] = rx;
        sizes[rx] += sizes[ry];
      } else {
        parents[rx] = ry;
        sizes[ry] += sizes[rx];
      }
    }
  }
  int GetSize(int x) { return sizes[x]; }
};

class Solution {
 public:
  long long countPairs(int n, vector<vector<int>> &edges) {
    UnionFind uf(n);
    for (const auto &edge : edges) {
      uf.Union(edge[0], edge[1]);
    }
    long long res = 0;
    for (int i = 0; i < n; i++) {
      res += n - uf.GetSize(uf.Find(i));
    }
    return res / 2;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, long long>> tests{
      {3, {{0, 1}, {0, 2}, {1, 2}}, 0},
      {7, {{0, 2}, {0, 5}, {2, 4}, {1, 6}, {5, 4}}, 14},
  };

  for (auto &[n, edges, ans] : tests) {
    assert(Solution().countPairs(n, edges) == ans);
  }
}