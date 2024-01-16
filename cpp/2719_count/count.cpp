/*
 * @Date: 2024-01-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-16
 * @FilePath: /algorithm/cpp/2719_count/count.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
  static constexpr int N = 23;
  static constexpr int M = 401;
  static constexpr int MOD = 1e9 + 7;
  int d[N][M];
  string _num;
  int _min_sum;
  int _max_sum;

  int dfs(int i, int j, bool limit) {
    if (j > _max_sum) {
      return 0;
    }
    if (i == -1) {
      return j >= _min_sum;
    }
    if (!limit && d[i][j] != -1) {
      return d[i][j];
    }
    int res = 0;
    int up = limit ? _num[i] - '0' : 9;
    for (int x = 0; x <= up; x++) {
      res = (res + dfs(i - 1, j + x, limit && x == up)) % MOD;
    }
    if (!limit) {
      d[i][j] = res;
    }
    return res;
  }

  int get(string n) {
    reverse(n.begin(), n.end());
    this->_num = n;
    return dfs(n.size() - 1, 0, true);
  }

  // 求解 num - 1，先把最后一个非 0 字符减去 1，再把后面的 0 字符变为 9
  string sub(string n) {
    int i = n.size() - 1;
    while (n[i] == '0') {
      i--;
    }
    n[i]--;
    i++;
    while (i < n.size()) {
      n[i] = '9';
      i++;
    }
    return n;
  }

 public:
  int count(string num1, string num2, int min_sum, int max_sum) {
    memset(d, -1, sizeof d);
    this->_min_sum = min_sum;
    this->_max_sum = max_sum;
    return (get(num2) - get(sub(num1)) + MOD) % MOD;
  }
};

int main() {
  vector<tuple<string, string, int, int, int>> tests{
      {"1", "12", 1, 8, 11},
      {"1", "5", 1, 5, 5},
  };

  for (auto &[num1, num2, min_sum, max_sum, ans] : tests) {
    assert(Solution().count(num1, num2, min_sum, max_sum) == ans);
  }
}