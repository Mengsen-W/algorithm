/*
 * @Date: 2022-10-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-14
 * @FilePath: /algorithm/940_distinct_subseq_ii/distinct_subseq_ii.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int distinctSubseqII(string s) {
    vector<int> last(26, -1);

    int n = s.size();
    vector<int> f(n, 1);
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < 26; ++j) {
        if (last[j] != -1) {
          f[i] = (f[i] + f[last[j]]) % mod;
        }
      }
      last[s[i] - 'a'] = i;
    }

    int ans = 0;
    for (int i = 0; i < 26; ++i) {
      if (last[i] != -1) {
        ans = (ans + f[last[i]]) % mod;
      }
    }
    return ans;
  }

 private:
  static constexpr int mod = 1000000007;
};

int main() {
  assert(Solution().distinctSubseqII("abc") == 7);
  assert(Solution().distinctSubseqII("aba") == 6);
  assert(Solution().distinctSubseqII("aaa") == 3);
}