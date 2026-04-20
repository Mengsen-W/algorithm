#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxDistance(vector<int>& colors) {
    int n = colors.size();
    int res = 0;  // 两栋颜色不同房子的最远距离
    // 遍历两栋房子下标并维护最远距离
    for (int i = 0; i < n; ++i) {
      for (int j = i + 1; j < n; ++j) {
        if (colors[i] != colors[j]) {
          res = max(res, j - i);
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 1, 1, 6, 1, 1, 1}, 3},
      {{1, 8, 3, 8, 3}, 4},
      {{0, 1}, 1},
  };

  for (auto& [colors, ans] : tests) {
    assert(Solution().maxDistance(colors) == ans);
  }
}
