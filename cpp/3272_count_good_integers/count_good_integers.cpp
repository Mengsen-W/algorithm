#include <cassert>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countGoodIntegers(int n, int k) {
    unordered_set<string> dict;
    int base = pow(10, (n - 1) / 2);
    int skip = n & 1;
    /* 枚举 n 个数位的回文数 */
    for (int i = base; i < base * 10; i++) {
      string s = to_string(i);
      s += string(s.rbegin() + skip, s.rend());
      long long palindromicInteger = stoll(s);
      /* 如果当前回文数是 k 回文数 */
      if (palindromicInteger % k == 0) {
        sort(s.begin(), s.end());
        dict.emplace(s);
      }
    }

    vector<long long> factorial(n + 1, 1);
    long long ans = 0;
    for (int i = 1; i <= n; i++) {
      factorial[i] = factorial[i - 1] * i;
    }
    for (const string &s : dict) {
      vector<int> cnt(10);
      for (char c : s) {
        cnt[c - '0']++;
      }
      /* 计算排列组合 */
      long long tot = (n - cnt[0]) * factorial[n - 1];
      for (int x : cnt) {
        tot /= factorial[x];
      }
      ans += tot;
    }

    return ans;
  }
};

int main() {
  vector<tuple<int, int, long long>> tests{
      {3, 5, 27},
      {1, 4, 2},
      {5, 6, 2468},
  };

  for (auto &[n, k, ans] : tests) {
    assert(Solution().countGoodIntegers(n, k) == ans);
  }
}