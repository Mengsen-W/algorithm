#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int longestBalanced(string s) {
    int n = s.size();
    int res = 0;
    vector<int> cnt(26);
    for (int i = 0; i < n; i++) {
      fill(cnt.begin(), cnt.end(), 0);
      for (int j = i; j < n; j++) {
        bool flag = true;
        int c = s[j] - 'a';
        cnt[c]++;
        for (auto x : cnt) {
          if (x > 0 && x != cnt[c]) {
            flag = false;
            break;
          }
        }
        if (flag) {
          res = max(res, j - i + 1);
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"abbac", 4},
      {"zzabccy", 4},
      {"aba", 2},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().longestBalanced(s) == ans);
    assert(Solution().longestBalanced())
  }
}
