#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  bool checkPowersOfThree(int n) {
    while (n) {
      if (n % 3 == 2) {
        return false;
      }
      n /= 3;
    }
    return true;
  }
};

int main() {
  std::vector<std::tuple<int, bool>> tests{
      {12, true},
      {91, true},
      {21, false},
  };

  for (auto& [n, expected] : tests) {
    assert(Solution().checkPowersOfThree(n) == expected);
  }
}