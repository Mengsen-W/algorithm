#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minDeletionSize(vector<string> strs) {
    int row = strs.size();
    int col = strs[0].size();
    int ans = 0;
    for (int j = 0; j < col; ++j) {
      for (int i = 1; i < row; ++i) {
        if (strs[i - 1][j] > strs[i][j]) {
          ans++;
          break;
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<string>, int>> tests{
      {{"cba", "daf", "ghi"}, 1},
      {{"a", "b"}, 0},
      {{"zyx", "wvu", "tsr"}, 3},
  };

  for (auto& [strs, expect] : tests) {
    assert(Solution().minDeletionSize(strs) == expect);
  }
}