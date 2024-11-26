#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfAlternatingGroups(vector<int>& colors) {
    int n = colors.size();
    int res = 0;
    for (int i = 0; i < n; i++) {
      if (colors[i] != colors[(i - 1 + n) % n] && colors[i] != colors[(i + 1) % n]) {
        res += 1;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 1, 1}, 0},
      {{0, 1, 0, 0, 1}, 3},
  };

  for (auto &[colors, ans] : tests) {
    assert(Solution().numberOfAlternatingGroups(colors) == ans);
  }
}