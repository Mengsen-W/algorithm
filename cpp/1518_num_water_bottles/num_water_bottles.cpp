#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int numWaterBottles(int numBottles, int numExchange) {
    return numBottles >= numExchange ? (numBottles - numExchange) / (numExchange - 1) + 1 + numBottles : numBottles;
  }
};

int main() {
  std::vector<std::tuple<int, int, int>> test_cases{
      {9, 3, 13},
      {15, 4, 19},
      {5, 5, 6},
      {2, 3, 2},
  };

  for (const auto& [numBottles, numExchange, expected] : test_cases) {
    assert(Solution().numWaterBottles(numBottles, numExchange) == expected);
  }
}