#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

class Solution {
 public:
  int countTriples(int n) {
    int res = 0;
    // 枚举 a 与 b
    for (int a = 1; a <= n; ++a) {
      for (int b = 1; b <= n; ++b) {
        // 判断是否符合要求
        int c = int(std::sqrt(a * a + b * b + 1.0));
        if (c <= n && c * c == a * a + b * b) {
          ++res;
        }
      }
    }
    return res;
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {5, 2},
      {10, 4},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().countTriples(n) == ans);
  }
}