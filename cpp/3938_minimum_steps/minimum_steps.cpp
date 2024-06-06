#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long minimumSteps(string s) {
    long long ans = 0;
    int sum = 0;
    for (int i = 0; i < s.size(); i++) {
      if (s[i] == '1') {
        sum++;
      } else {
        ans += sum;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, long long>> tests{
      {"101", 1},
      {"100", 2},
      {"0111", 0},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().minimumSteps(s) == ans);
  }
}