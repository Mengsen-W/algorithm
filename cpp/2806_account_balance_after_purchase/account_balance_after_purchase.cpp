#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int accountBalanceAfterPurchase(int purchaseAmount) { return 100 - (purchaseAmount + 5) / 10 * 10; }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {9, 90},
      {15, 80},
  };

  for (auto &[purchaseAmount, ans] : tests) {
    assert(Solution().accountBalanceAfterPurchase(purchaseAmount) == ans);
  }
}