#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  bool checkDivisibility(int n) {
    int digitSum = 0;
    int digitProduct = 1;
    int original = n;

    while (n > 0) {
      int digit = n % 10;
      n /= 10;

      digitSum += digit;
      digitProduct *= digit;
    }

    return original % (digitSum + digitProduct) == 0;
  }
};

int main() {
  std::vector<std::tuple<int, bool>> tests{
      {99, true},
      {23, false},
  };

  for (auto& [n, ans] : tests) {
    assert(Solution().checkDivisibility(n) == ans);
  }
}