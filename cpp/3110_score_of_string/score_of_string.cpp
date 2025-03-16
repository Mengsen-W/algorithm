#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int scoreOfString(string s) {
    int n = s.size();
    int score = 0;
    for (int i = 0; i + 1 < n; ++i) {
      score += abs(s[i] - s[i + 1]);
    }
    return score;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"hello", 13},
      {"zaz", 50},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().scoreOfString(s) == ans);
  }
}