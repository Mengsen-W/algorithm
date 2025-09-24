#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximum69Number(int num) {
    int digitBase = pow(10, (int)log10(num));
    while (digitBase > 0) {
      if ((num / digitBase) % 10 == 6) {
        num += 3 * digitBase;
        return num;
      }
      digitBase /= 10;
    }

    return num;
  }
};

int main() {
  vector<tuple<int, int>> tests{
      {9669, 9969},
      {9996, 9999},
      {9999, 9999},
  };

  for (auto [input, expected] : tests) {
    assert(Solution().maximum69Number(input) == expected);
  }
}