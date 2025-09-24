#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimizedStringLength(string s) {
    int mask = 0;
    for (char c : s) {
      mask |= 1 << (c - 'a');
    }
    return __builtin_popcount(mask);
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"aaabc", 3},
      {"cbbd", 3},
      {"dddaaa", 2},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().minimizedStringLength(s) == ans);
  }
}