#include <array>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int maximumLengthSubstring(string s) {
    array<int, 26> count{};
    int left = 0;
    int res = 0;
    for (int right = 0; right < s.length(); ++right) {
      int ch = s[right] - 'a';
      ++count[ch];
      while (count[ch] > 2) {
        const int ch2 = s[left] - 'a';
        --count[ch2];
        ++left;
      }
      res = max(res, right - left + 1);
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"bcbbbcba", 4},
      {"aaaa", 2},
  };

  for (const auto& [s, ans] : tests) {
    assert(Solution().maximumLengthSubstring(s) == ans);
  }
}