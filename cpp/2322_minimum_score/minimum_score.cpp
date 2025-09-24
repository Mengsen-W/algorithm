#include <cassert>
#include <climits>
#include <functional>
#include <vector>

using namespace std;

class Solution {
 public:
  int calc(int part1, int part2, int part3) { return max(part1, max(part2, part3)) - min(part1, min(part2, part3)); }

  int minimumScore(vector<int> &nums, vector<vector<int>> &edges) {
    int n = nums.size();
    vector<vector<int>> e(n);
    for (auto &v : edges) {
      e[v[0]].push_back(v[1]);
      e[v[1]].push_back(v[0]);
    }

    int sum = 0;
    for (int x : nums) {
      sum ^= x;
    }

    int res = INT_MAX;
    function<int(int, int, int, int)> dfs2 = [&](int x, int f, int oth, int anc) {
      int son = nums[x];
      for (auto &y : e[x]) {
        if (y == f) {
          continue;
        }
        son ^= dfs2(y, x, oth, anc);
      }
      if (f == anc) {
        return son;
      }
      res = min(res, calc(oth, son, sum ^ oth ^ son));
      return son;
    };

    function<int(int, int)> dfs = [&](int x, int f) {
      int son = nums[x];
      for (auto &y : e[x]) {
        if (y == f) {
          continue;
        }
        son ^= dfs(y, x);
      }

      for (auto &y : e[x]) {
        if (y == f) {
          dfs2(y, x, son, x);
        }
      }
      return son;
    };

    dfs(0, -1);
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>, int>> tests {
    {{1,5,5,4,11}, {{0,1},{1,2},{1,3},{3,4}}, 9 },
    {{5,5,2,4,4,2}, {{0,1},{1,2},{5,2},{4,3},{1,3}}, 0},
  };

  for (auto & [nums, edges, expect] : tests) {
    assert(Solution().minimumScore(nums, edges) == expect);
  }
}