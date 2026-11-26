#include <cassert>
#include <cmath>
#include <vector>
#include <tuple>
using namespace std;

class Solution {
public:
    bool canAliceWin(int n) {
        int k = (21 - sqrt(441 - 8 * n)) / 2;
        return k % 2 == 1;
    }
};

int main() {
  vector<tuple<int, bool>> tests{
      {12, true},
      {1, false},
  };

  for (auto [n, expected] : tests) {
    assert(Solution().canAliceWin(n) == expected);
  }
}