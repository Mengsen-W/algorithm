#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int countSymmetricIntegers(int low, int high) {
    int res = 0;
    for (int a = low; a <= high; ++a) {
      if (a < 100 && a % 11 == 0) {
        res++;
      } else if (1000 <= a && a < 10000) {
        int left = a / 1000 + (a % 1000) / 100;
        int right = (a % 100) / 10 + a % 10;
        if (left == right) {
          res++;
        }
      }
    }
    return res;
  }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests = {
      {1, 100, 9},
      {1200, 1230, 4},
  };

  for (auto &[low, high, ans] : tests) {
    assert(Solution().countSymmetricIntegers(low, high) == ans);
  }
}