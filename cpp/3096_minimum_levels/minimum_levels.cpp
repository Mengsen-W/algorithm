#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumLevels(vector<int>& possible) {
    int n = possible.size();
    int tot = accumulate(possible.begin(), possible.end(), 0) * 2 - n;
    int pre = 0;
    for (int i = 0; i < n - 1; i++) {
      pre += possible[i] == 1 ? 1 : -1;
      if (2 * pre > tot) {
        return i + 1;
      }
    }
    return -1;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests {
    {{1,0,1,0}, 1},
    {{1,1,1,1,1},3},
    {{0,0}, -1},
  };

  for (auto &[possible, ans] : tests) {
    assert(Solution().minimumLevels(possible) == ans);
  }
}