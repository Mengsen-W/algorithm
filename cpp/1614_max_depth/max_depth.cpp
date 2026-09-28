#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxDepth(string s) {
    int ans = 0, size = 0;
    for (char ch : s) {
      if (ch == '(') {
        ++size;
        ans = max(ans, size);
      } else if (ch == ')') {
        --size;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"(1+(2*3)+((8)/4))+1", 3},
      {"(1)+((2))+(((3)))", 3},
      {"1+(2*3)/(2-1)", 1},
      {"1", 0},
  };


  for (auto& [s, expected] : tests) {
    assert(Solution().maxDepth(s) == expected);
  }
}