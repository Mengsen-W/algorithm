#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int maxProduct(int n) {
    int first = 0, second = 0;
    while (n > 0) {
      int x = n % 10;
      if (x > first) {
        second = first;
        first = x;
      } else if (x > second) {
        second = x;
      }
      n /= 10;
    }
    return first * second;
  }
};

int main() {
  vector<tuple<int, int>> tests{
      {31, 3},
      {22, 4},
      {124, 8},
  };

  for (auto [n, expected] : tests) {
    assert(Solution().maxProduct(n) == expected);
  }
  return 0;
}