/*
 * @Date: 2021-09-13 08:20:50
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-08
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfBoomerangs(vector<vector<int>> &points) {
    int ans = 0;
    for (auto &p : points) {
      unordered_map<int, int> cnt;
      for (auto &q : points) {
        int dis = (p[0] - q[0]) * (p[0] - q[0]) + (p[1] - q[1]) * (p[1] - q[1]);
        ++cnt[dis];
      }
      for (auto &[_, m] : cnt) ans += m * (m - 1);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{0, 0}, {1, 0}, {2, 0}}, 2},
      {{{1, 1}, {2, 2}, {3, 3}}, 2},
      {{{1, 1}}, 0},
  };

  for (auto &[points, ans] : tests) {
    assert(Solution().numberOfBoomerangs(points) == ans);
  }
}