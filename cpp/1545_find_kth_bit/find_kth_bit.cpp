#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  char findKthBit(int n, int k) {
    if (k == 1) {
      return '0';
    }
    int mid = 1 << (n - 1);
    if (k == mid) {
      return '1';
    } else if (k < mid) {
      return findKthBit(n - 1, k);
    } else {
      k = mid * 2 - k;
      return invert(findKthBit(n - 1, k));
    }
  }

  char invert(char bit) { return (char)('0' + '1' - bit); }
};

int main() {
  std::vector<std::tuple<int, int, char>> tests{
      {3, 1, '0'},
      {4, 11, '1'},
      {1, 1, '0'},
      {2, 3, '1'},
  };

  for (auto [n, k, expected] : tests) {
    assert(Solution().findKthBit(n, k) == expected);
  }
}