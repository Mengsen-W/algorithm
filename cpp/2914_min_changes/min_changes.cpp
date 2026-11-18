#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    int minChanges(string s) {
        int ans = 0;
        for (int i = 0; i < s.size(); i += 2) {
            ans += s[i] != s[i + 1];
        }
        return ans;
    }
};

int main() {
  vector<tuple<string, int>> tests{
      {"1001", 2},
      {"10", 1},
      {"0000", 0},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().minChanges(s) == expected);
  }
}