#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numSteps(string s) {
    int n = s.size();
    int ans = 0;
    // meet1 记录我们有没有遇见过字符 1
    bool meet1 = false;
    // 从后向前遍历字符
    for (int i = n - 1; i >= 0; --i) {
      if (s[i] == '0') {
        ans += (meet1 ? 2 : 1);
      } else {
        if (!meet1) {
          if (i != 0) {
            ans += 2;
          }
          meet1 = true;
        } else {
          ++ans;
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"1101", 6},
      {"10", 1},
      {"1", 0},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().numSteps(s) == expected);
  }
}