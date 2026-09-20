#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int reverseDegree(string s) {
    int ans = 0;
    for (int i = 1; i <= s.size(); i++) {
      ans += (26 - (s[i - 1] - 'a')) * i;
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"abc", 148},
      {"zaza", 160},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().reverseDegree(s) == ans);
  }
  return 0;
}