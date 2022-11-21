/*
 * @Date: 2022-11-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-21
 * @FilePath: /algorithm/808_soup_servings/soup_servings.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  double soupServings(int n) {
    n = ceil((double)n / 25);
    if (n >= 179) {
      return 1.0;
    }
    memo = vector<vector<double>>(n + 1, vector<double>(n + 1));
    return dfs(n, n);
  }

  double dfs(int a, int b) {
    if (a <= 0 && b <= 0) {
      return 0.5;
    } else if (a <= 0) {
      return 1;
    } else if (b <= 0) {
      return 0;
    }
    if (memo[a][b] > 0) {
      return memo[a][b];
    }
    memo[a][b] = 0.25 * (dfs(a - 4, b) + dfs(a - 3, b - 1) + dfs(a - 2, b - 2) + dfs(a - 1, b - 3));
    return memo[a][b];
  }

 private:
  vector<vector<double>> memo;
};

int main() {
  assert(Solution().soupServings(50) == 0.62500);
  assert(Solution().soupServings(100) == 0.71875);
}