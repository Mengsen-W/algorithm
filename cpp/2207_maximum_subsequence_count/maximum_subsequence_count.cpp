#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maximumSubsequenceCount(string text, string pattern) {
    long long res = 0;
    int cnt1 = 0, cnt2 = 0;
    for (char c : text) {
      if (c == pattern[1]) {
        res += cnt1;
        cnt2++;
      }
      if (c == pattern[0]) {
        cnt1++;
      }
    }
    return res + max(cnt1, cnt2);
  }
};

int main() {
  vector<tuple<string, string, long long>> tests{
      {"abdcdbc", "ac", 4},
      {"aabb", "ab", 6},
  };

  for (auto &[text, pattern, ans] : tests) {
    assert(Solution().maximumSubsequenceCount(text, pattern) == ans);
  }
}