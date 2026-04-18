#include <cassert>
#include <cstdlib>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int reverse(int n) {
    int res = 0;
    while (n > 0) {
      res = res * 10 + n % 10;
      n /= 10;
    }
    return res;
  }

  int mirrorDistance(int n) { return abs(n - reverse(n)); }
};

int main() {
  vector<tuple<int, int>> tests{
      {25, 27},
      {10, 9},
      {7, 0},
  };

  for (auto [n, expected] : tests) {
    assert(Solution().mirrorDistance(n) == expected);
  }
}