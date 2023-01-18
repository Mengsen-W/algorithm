/*
 * @Date: 2022-12-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-10
 * @FilePath: /algorithm/1691_max_height/max_height.cpp
 */

#include <cassert>
#include <functional>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxHeight(vector<vector<int>> &cuboids) {
    int n = cuboids.size();
    for (auto &v : cuboids) {
      sort(v.begin(), v.end());
    }
    sort(cuboids.begin(), cuboids.end(),
         [](const vector<int> &a, const vector<int> &b) { return a[0] + a[1] + a[2] < b[0] + b[1] + b[2]; });

    vector<int> memo(n, -1);
    function<bool(const vector<int> &a, const vector<int> &b)> check = [](const vector<int> &a, const vector<int> &b) {
      return a[0] <= b[0] && a[1] <= b[1] && a[2] <= b[2];
    };
    function<int(int, int)> dfs = [&](int top, int index) -> int {
      if (index == cuboids.size()) {
        return 0;
      }
      if (top != -1 && memo[top] != -1) {
        return memo[top];
      }
      int height = dfs(top, index + 1);
      if (top == -1 || check(cuboids[top], cuboids[index])) {
        height = max(height, cuboids[index][2] + dfs(index, index + 1));
      }
      if (top != -1) {
        memo[top] = height;
      }
      return height;
    };
    return dfs(-1, 0);
  }
};

int main() {
  {
    vector<vector<int>> cuboids{{50, 45, 20}, {95, 37, 53}, {45, 23, 12}};
    assert(Solution().maxHeight(cuboids) == 190);
  }

  {
    vector<vector<int>> cuboids{{38, 25, 45}, {76, 35, 3}};
    assert(Solution().maxHeight(cuboids) == 76);
  }

  {
    vector<vector<int>> cuboids{{7, 11, 17}, {7, 17, 11}, {11, 7, 17}, {11, 17, 7}, {17, 7, 11}, {17, 11, 7}};
    assert(Solution().maxHeight(cuboids) == 102);
  }
}