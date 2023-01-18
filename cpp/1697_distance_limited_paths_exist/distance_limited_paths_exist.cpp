/*
 * @Date: 2022-12-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-14
 * @FilePath: /algorithm/1697_distance_limited_paths_exist/distance_limited_paths_exist.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int find(vector<int> &uf, int x) {
    if (uf[x] == x) {
      return x;
    }
    return uf[x] = find(uf, uf[x]);
  }

  void merge(vector<int> &uf, int x, int y) {
    x = find(uf, x);
    y = find(uf, y);
    uf[y] = x;
  }

  vector<bool> distanceLimitedPathsExist(int n, vector<vector<int>> &edgeList, vector<vector<int>> &queries) {
    sort(edgeList.begin(), edgeList.end(), [](vector<int> &e1, vector<int> &e2) { return e1[2] < e2[2]; });

    vector<int> index(queries.size());
    iota(index.begin(), index.end(), 0);
    sort(index.begin(), index.end(), [&](int i1, int i2) { return queries[i1][2] < queries[i2][2]; });

    vector<int> uf(n);
    iota(uf.begin(), uf.end(), 0);
    vector<bool> res(queries.size());
    int k = 0;
    for (auto i : index) {
      while (k < edgeList.size() && edgeList[k][2] < queries[i][2]) {
        merge(uf, edgeList[k][0], edgeList[k][1]);
        k++;
      }
      res[i] = find(uf, queries[i][0]) == find(uf, queries[i][1]);
    }
    return res;
  }
};

int main() {
  {
    int n = 3;
    vector<vector<int>> edgeList{{0, 1, 2}, {1, 2, 4}, {2, 0, 8}, {1, 0, 16}};
    vector<vector<int>> queries{{0, 1, 2}, {0, 2, 5}};
    vector<bool> ans{false, true};
    assert(Solution().distanceLimitedPathsExist(n, edgeList, queries) == ans);
  }

  {
    int n = 5;
    vector<vector<int>> edgeList{{0, 1, 10}, {1, 2, 5}, {2, 3, 9}, {3, 4, 13}};
    vector<vector<int>> queries{{0, 4, 14}, {1, 4, 13}};
    vector<bool> ans{true, false};
    assert(Solution().distanceLimitedPathsExist(n, edgeList, queries) == ans);
  }
}
