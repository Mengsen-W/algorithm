#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minDeletionSize(vector<string>& strs) {
    // cuts[i] is True : we don't need to check col[i] <= col[i+1]
    vector<bool> cuts(strs.size() - 1, false);
    int ans = 0;
    for (size_t j = 0; j < strs[0].size(); j++) {
      bool canKeep = true;
      for (size_t i = 0; i < strs.size() - 1; i++) {
        if (!cuts[i] && strs[i][j] > strs[i + 1][j]) {
          canKeep = false;
          break;
        }
      }
      if (canKeep) {
        for (size_t i = 0; i < strs.size() - 1; i++) {
          if (strs[i][j] < strs[i + 1][j]) {
            cuts[i] = true;
          }
        }
      } else {
        ans++;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<string>, int>> tests{
      {{"ca", "bb", "ac"}, 1},
      {{"xc", "yb", "za"}, 0},
      {{"zyx", "wvu", "tsr"}, 3},
  };

  for (auto& [strs, ans] : tests) {
    assert(Solution().minDeletionSize(strs) == ans);
  }
}