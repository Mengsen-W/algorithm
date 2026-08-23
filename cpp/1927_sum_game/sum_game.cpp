#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  bool sumGame(string num) {
    int n = num.size();

    auto get = [](string&& s) -> pair<int, int> {
      int nn = 0, qq = 0;
      for (char ch : s) {
        if (ch == '?') {
          ++qq;
        } else {
          nn += (ch - '0');
        }
      }
      return {nn, qq};
    };

    auto [n0, q0] = get(num.substr(0, n / 2));
    auto [n1, q1] = get(num.substr(n / 2, n / 2));

    return ((q0 + q1) % 2 == 1) || (n0 - n1 != (q1 - q0) * 9 / 2);
  }
};

int main() {
  vector<tuple<string, bool>> tests{
      {"5023", false},
      {"25??", true},
      {"?3295???", false},
  };

  for (auto& [num, ans] : tests) {
    assert(Solution().sumGame(num) == ans);
  }
}