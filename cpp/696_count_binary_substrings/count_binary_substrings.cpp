#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int countBinarySubstrings(string s) {
    int ptr = 0, n = s.size(), last = 0, ans = 0;
    while (ptr < n) {
      char c = s[ptr];
      int count = 0;
      while (ptr < n && s[ptr] == c) {
        ++ptr;
        ++count;
      }
      ans += min(count, last);
      last = count;
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"00110011", 6},
      {"10101", 4},
  };

  for (auto& [s, ans] : tests) {
    assert(Solution().countBinarySubstrings(s) == ans);
  }
  return 0;
}
