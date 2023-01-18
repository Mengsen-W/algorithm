/*
 * @Date: 2021-09-13 08:20:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-13 08:31:34
 */

#include <cassert>
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
  {
    vector<vector<int>> points{{0, 0}, {1, 0}, {2, 0}};
    int ans = 2;
    assert(Solution{}.numberOfBoomerangs(points) == ans);
  }
  {
    vector<vector<int>> points{{1, 1}, {2, 2}, {3, 3}};
    int ans = 2;
    assert(Solution{}.numberOfBoomerangs(points) == ans);
  }
  {
    vector<vector<int>> points{{1, 1}};
    int ans = 0;
    assert(Solution{}.numberOfBoomerangs(points) == ans);
  }
}