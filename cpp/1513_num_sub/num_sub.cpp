#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  static constexpr int P = int(1E9) + 7;

  int numSub(string s) {
    int p = 0;
    long long ans = 0;
    while (p < s.size()) {
      if (s[p] == '0') {
        ++p;
        continue;
      }
      int cnt = 0;
      while (p < s.size() && s[p] == '1') {
        ++cnt;
        ++p;
      }
      ans = ans + (1LL + (long long)cnt) * cnt / 2;
      ans = ans % P;
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"0110111", 9},
      {"101", 2},
      {"111111", 21},
      {"000", 0},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().numSub(s) == ans);
  }
}