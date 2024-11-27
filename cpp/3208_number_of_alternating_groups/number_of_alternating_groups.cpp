#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfAlternatingGroups(vector<int>& colors, int k) {
    int n = colors.size();
    int res = 0, cnt = 1;
    for (int i = -k + 2; i < n; i++) {
      if (colors[(i + n) % n] != colors[(i - 1 + n) % n]) {
        cnt += 1;
      } else {
        cnt = 1;
      }
      if (cnt >= k) {
        res += 1;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{0, 1, 0, 1, 0}, 3, 3},
      {{0, 1, 0, 0, 1, 0, 1}, 6, 2},
      {{1, 1, 0, 1}, 4, 0},
  };

  for (auto &[colors, k, ans] : tests) {
    assert(Solution().numberOfAlternatingGroups(colors, k) == ans);
  }
}