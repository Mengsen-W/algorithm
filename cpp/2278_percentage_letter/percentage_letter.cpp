
#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int percentageLetter(string s, char letter) { return ranges::count(s, letter) * 100 / s.size(); }
};

int main() {
  vector<tuple<string, char, int>> tests{
      {"foobar", 'o', 33},
      {"jjjj", 'k', 0},
  };

  for (auto& [s, letter, ans] : tests) {
    assert(Solution().percentageLetter(s, letter) == ans);
  }
}