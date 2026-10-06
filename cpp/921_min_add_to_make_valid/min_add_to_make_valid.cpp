#include <cassert>
#include <string>
#include <tuple>
#include <vector>

class Solution {
 public:
  int minAddToMakeValid(std::string s) {
    // ans total right, cnt total remind left
    int ans = 0, cnt = 0;
    for (auto &c : s) {
      if (c == '(') {
        cnt++;
      } else {
        if (cnt > 0) {
          cnt--;
        } else {
          ans++;
        }
      }
    }
    return ans + cnt;
  }
};

int main() {
  std::vector<std::tuple<std::string, int>> tests{
      {"())", 1},
      {"(((", 3},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().minAddToMakeValid(s) == ans);
  }
}