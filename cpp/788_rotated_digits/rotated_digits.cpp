/*
 * @Date: 2022-09-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-25
 * @FilePath: /algorithm/788_rotated_digits/rotated_digits.cpp
 */

#include <assert.h>

#include <functional>
#include <vector>

using namespace std;

class Solution {
 public:
  int rotatedDigits(int n) {
    vector<int> digits;
    while (n) {
      digits.push_back(n % 10);
      n /= 10;
    }
    reverse(digits.begin(), digits.end());

    memset(memo, -1, sizeof(memo));
    function<int(int, bool, bool)> dfs = [&](int pos, bool bound, bool diff) -> int {
      if (pos == digits.size()) {
        return diff;
      }
      if (memo[pos][bound][diff] != -1) {
        return memo[pos][bound][diff];
      }

      int ret = 0;
      for (int i = 0; i <= (bound ? digits[pos] : 9); ++i) {
        if (check[i] != -1) {
          ret += dfs(pos + 1, bound && (i == digits[pos]), diff || (check[i] == 1));
        }
      }
      return memo[pos][bound][diff] = ret;
    };

    int ans = dfs(0, true, false);
    return ans;
  }

 private:
  static constexpr int check[10] = {0, 0, 1, -1, -1, 1, 1, -1, 0, 1};
  int memo[5][2][2];
};

int main() { assert(Solution().rotatedDigits(10) == 4); }