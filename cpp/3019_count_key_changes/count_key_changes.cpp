#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countKeyChanges(string s) {
    int ans = 0;
    for (int i = 1; i < s.size(); ++i) {
      if (tolower(s[i - 1]) != tolower(s[i])) {
        ++ans;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"aAbBcC", 2},
      {"AaAaAaaA", 0},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().countKeyChanges(s) == ans);
  }
}