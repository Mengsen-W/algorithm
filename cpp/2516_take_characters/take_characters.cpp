
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int takeCharacters(string s, int k) {
    vector<int> cnt(3, 0);
    int len = s.size();
    int ans = len;
    for (int i = 0; i < len; i++) {
      cnt[s[i] - 'a']++;
    }
    if (cnt[0] >= k && cnt[1] >= k && cnt[2] >= k) {
      ans = min(ans, len);
    } else {
      return -1;
    }

    int l = 0;
    for (int r = 0; r < len; r++) {
      cnt[s[r] - 'a']--;
      while (l < r && (cnt[0] < k || cnt[1] < k || cnt[2] < k)) {
        cnt[s[l] - 'a']++;
        l++;
      }
      if (cnt[0] >= k && cnt[1] >= k && cnt[2] >= k) {
        ans = min(ans, len - (r - l + 1));
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<string, int, int>> tests{
      {"aabaaaacaabc", 2, 8},
      {"a", 1, -1},
  };

  for (auto &[s, k, ans] : tests) {
    assert(Solution().takeCharacters(s, k) == ans);
  }
}