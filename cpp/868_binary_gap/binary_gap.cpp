#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int binaryGap(int n) {
    int last = -1, ans = 0;
    for (int i = 0; n; ++i) {
      if (n & 1) {
        if (last != -1) {
          ans = max(ans, i - last);
        }
        last = i;
      }
      n >>= 1;
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, int>> tests{
      {22, 2},
      {8, 0},
      {5, 2},
  };

  for (auto& [n, ans] : tests) {
    assert(Solution().binaryGap(n) == ans);
  }
}
