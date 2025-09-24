#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string reverseStr(string s, int k) {
    int n = s.length();
    for (int i = 0; i < n; i += 2 * k) {
      reverse(s.begin() + i, s.begin() + min(i + k, n));
    }
    return s;
  }
};

int main() {
  vector<tuple<string, int, string>> tests{
      {"abcdefg", 2, "bacdfeg"},
      {"abcd", 2, "bacd"},
  };

  for (auto &[s, k, ans] : tests) {
    assert(Solution().reverseStr(s, k) == ans);
  }
}