#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int maxBottlesDrunk(int numBottles, int numExchange) {
    int ans = numBottles;
    for (int empty = numBottles; empty >= numExchange; numExchange++) {
      ans++;
      empty -= numExchange - 1;
    }
    return ans;
  }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {13, 6, 15},
      {10, 3, 13},
  };

  for (auto &[numBottles, numExchange, ans] : tests) {
    assert(Solution().maxBottlesDrunk(numBottles, numExchange) == ans);
  }
}