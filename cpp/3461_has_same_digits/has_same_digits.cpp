#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool hasSameDigits(string s) {
    int n = s.length();
    for (int i = 1; i <= n - 2; i++) {
      for (int j = 0; j <= n - 1 - i; j++) {
        s[j] = ((s[j] - '0') + (s[j + 1] - '0')) % 10 + '0';
      }
    }
    return s[0] == s[1];
  }
};

int main() {
  vector<tuple<string, bool>> tests{
      {"3902", true},
      {"34789", false},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().hasSameDigits(s) == ans);
  }
}