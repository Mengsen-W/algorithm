#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool doesAliceWin(string s) {
    return ranges::any_of(s, [](char c) { return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'; });
  }
};

int main() {
  vector<tuple<string, bool>> tests{
      {"leetcoder", true},
      {"bbcd", false},
  };

  for (auto [s, expect] : tests) {
    assert(Solution().doesAliceWin(s) == expect);
  }
}