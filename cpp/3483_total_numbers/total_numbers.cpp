#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int totalNumbers(vector<int>& digits) {
    int n = digits.size();
    bool vis[1000]{};
    int ans = 0;

    for (int i = 0; i < n; ++i) {
      if (digits[i] == 0) {
        continue;
      }
      for (int j = 0; j < n; ++j) {
        if (j == i) {
          continue;
        }
        for (int k = 0; k < n; ++k) {
          if (k == i || k == j || digits[k] % 2 != 0) {
            continue;
          }
          int x = digits[i] * 100 + digits[j] * 10 + digits[k];
          if (!vis[x]) {
            vis[x] = true;
            ++ans;
          }
        }
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 4}, 12},
      {{0, 2, 2}, 2},
      {{6, 6, 6}, 1},
      {{1, 3, 5}, 0},
  };

  for (auto& [digits, ans] : tests) {
    assert(Solution().totalNumbers(digits) == ans);
  }
}