#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfSubstrings(string s) {
    int len = s.length();
    int ans = 0;
    vector<int> cnt(3);

    for (int l = 0, r = -1; l < len;) {
      while (r < len && !(cnt[0] >= 1 && cnt[1] >= 1 && cnt[2] >= 1)) {
        r++;
        if (r == len) {
          break;
        }
        cnt[s[r] - 'a']++;
      }
      if (r < len) {
        ans += len - r;
      }
      cnt[s[l] - 'a']--;
      l++;
    }

    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"abcabc", 10},
      {"aaacb", 3},
      {"abc", 1},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().numberOfSubstrings(s) == expected);
  }
}