#include <algorithm>
#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> pathExistenceQueries(int n, vector<int>& nums, int maxDiff, vector<vector<int>>& queries) {
    vector<int> idx(n), pos(n), res;
    iota(idx.begin(), idx.end(), 0);
    sort(idx.begin(), idx.end(), [&](int a, int b) { return nums[a] < nums[b]; });
    for (int i = 0; i < n; i++) {
      pos[idx[i]] = i;
    }

    int m = 32 - __builtin_clz(n);
    vector<vector<int>> f(n, vector<int>(m));

    for (int i = 0, left = 0; i < n; i++) {
      while (nums[idx[i]] - nums[idx[left]] > maxDiff) left++;
      f[i][0] = left;
    }

    for (int j = 1; j < m; j++) {
      for (int i = 0; i < n; i++) {
        f[i][j] = f[f[i][j - 1]][j - 1];
      }
    }

    for (auto& q : queries) {
      auto [x, y] = pair(pos[q[0]], pos[q[1]]);
      if (x > y) {
        swap(x, y);
      }
      if (x == y) {
        res.push_back(0);
        continue;
      }

      int step = 0;
      for (int i = m - 1; i >= 0; i--) {
        if (f[y][i] > x) {
          y = f[y][i];
          step += 1 << i;
        }
      }

      res.push_back(f[y][0] <= x ? step + 1 : -1);
    }
    return res;
  }
};

int main() {
  vector<tuple<int, vector<int>, int, vector<vector<int>>, vector<int>>> tests{
      {4, {1, 8, 3, 4, 2}, 3, {{0, 3}, {2, 4}}, {1, 1}},
      {5, {5, 3, 1, 9, 10}, 2, {{0, 1}, {0, 2}, {2, 3}, {4, 3}}, {1, 2, -1, 1}},
      {3, {3, 6, 1}, 1, {{0, 0}, {0, 1}, {1, 2}}, {0, -1, -1}},
  };

  for (auto& [n, nums, maxDiff, queries, expected] : tests) {
    assert(Solution().pathExistenceQueries(n, nums, maxDiff, queries) == expected);
  }

  return 0;
}