#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string removeTrailingZeros(string num) { return num.substr(0, num.find_last_not_of('0') + 1); }
};

int main() {
  vector<tuple<string, string>> tests{
      {"51230100", "512301"},
      {"123", "123"},
  };

  for (auto &[num, ans] : tests) {
    assert(Solution().removeTrailingZeros(num) == ans);
  }
}