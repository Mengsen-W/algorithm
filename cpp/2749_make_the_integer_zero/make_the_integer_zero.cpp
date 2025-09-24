#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int makeTheIntegerZero(int num1, int num2) {
    int k = 1;
    while (true) {
      long long x = num1 - static_cast<long long>(num2) * k;
      if (x < k) {
        return -1;
      }
      if (k >= __builtin_popcountll(x)) {
        return k;
      }
      k++;
    }
  }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {3, -2, 3},
      {5, 7, -1},
  };

  for (auto [num1, num2, expect] : tests) {
    assert(Solution().makeTheIntegerZero(num1, num2) == expect);
  }
}