/*
 * @Date: 2023-10-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-26
 * @FilePath: /algorithm/cpp/2698_punishment_number/punishment_number.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool dfs(string &s, int pos, int tot, int target) {
    if (pos == s.size()) {
      return tot == target;
    }
    int sum = 0;
    for (int i = pos; i < s.size(); i++) {
      sum = sum * 10 + s[i] - '0';
      if (sum + tot > target) {
        break;
      }
      if (dfs(s, i + 1, sum + tot, target)) {
        return true;
      }
    }
    return false;
  }

  int punishmentNumber(int n) {
    int res = 0;
    for (int i = 1; i <= n; i++) {
      string s = to_string(i * i);
      if (dfs(s, 0, 0, i)) {
        res += i * i;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int>> tests{
      {10, 182},
      {37, 1478},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().punishmentNumber(n) == ans);
  }
}
