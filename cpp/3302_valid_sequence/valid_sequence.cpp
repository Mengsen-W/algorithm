#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> validSequence(string s, string t) {
    int n = s.length(), m = t.length();
    vector<int> suf(n + 1);
    suf[n] = m;
    for (int i = n - 1, j = m - 1; i >= 0; i--) {
      if (j >= 0 && s[i] == t[j]) {
        j--;
      }
      suf[i] = j + 1;
    }

    vector<int> ans(m);
    bool changed = false;  // 是否修改过
    for (int i = 0, j = 0; i < n; i++) {
      if (s[i] == t[j] || (!changed && suf[i + 1] <= j + 1)) {
        if (s[i] != t[j]) {
          changed = true;
        }
        ans[j++] = i;
        if (j == m) {
          return ans;
        }
      }
    }
    return {};
  }
};

int main() {
  vector<tuple<string, string, vector<int>>> tests{
      {"vbcca", "abc", {0, 1, 2}},
      {"bacdc", "abc", {1, 2, 4}},
      {"aaaaaa", "aaabc", {}},
      {"abc", "ab", {0, 1}},
  };

  for (auto& [s, t, expected] : tests) {
    assert(Solution().validSequence(s, t) == expected);
  }
  return 0;
}