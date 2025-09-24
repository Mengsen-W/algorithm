#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int minimumSum(int n, int k) {
    if (n <= k / 2) {
      return arithmeticSeriesSum(1, 1, n);
    } else {
      return arithmeticSeriesSum(1, 1, k / 2) + arithmeticSeriesSum(k, 1, n - k / 2);
    }
  }

 private:
  int arithmeticSeriesSum(int a1, int d, int n) { return (a1 + a1 + (n - 1) * d) * n / 2; }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {5, 4, 18},
      {2, 6, 3},
  };

  for (auto &[n, k, ans] : tests) {
    assert(Solution().minimumSum(n, k) == ans);
  }
}