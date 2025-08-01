#include <cassert>
#include <queue>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string longestSubsequenceRepeatedK(string s, int k) {
    auto check = [&](const string& t, int k) -> bool {
      int i = 0;
      for (char c : s) {
        if (c == t[i]) {
          i++;
          if (i == t.size()) {
            if (--k == 0) {
              return true;
            }
            i = 0;
          }
        }
      }
      return false;
    };
    int cnt[26] = {};
    for (char c : s) {
      cnt[c - 'a']++;
    }

    vector<char> cs;
    for (char c = 'a'; c <= 'z'; ++c) {
      if (cnt[c - 'a'] >= k) {
        cs.push_back(c);
      }
    }

    queue<string> q;
    q.push("");
    string ans;
    while (!q.empty()) {
      string cur = q.front();
      q.pop();
      for (char c : cs) {
        string nxt = cur + c;
        if (check(nxt, k)) {
          ans = nxt;
          q.push(nxt);
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int, string>> tests{
      {"letsleetcode", 2, "let"},
      {"bb", 2, "b"},
      {"ab", 2, ""},
  };

  for (const auto& [s, k, expected] : tests) {
    assert(Solution().longestSubsequenceRepeatedK(s, k) == expected);
  }
  return 0;
}