#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> generate(int numRows) {
    vector<vector<int>> ret(numRows);
    for (int i = 0; i < numRows; ++i) {
      ret[i].resize(i + 1);
      ret[i][0] = ret[i][i] = 1;
      for (int j = 1; j < i; ++j) {
        ret[i][j] = ret[i - 1][j] + ret[i - 1][j - 1];
      }
    }
    return ret;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>>> tests{
      {5, {{1}, {1, 1}, {1, 2, 1}, {1, 3, 3, 1}, {1, 4, 6, 4, 1}}},
      {1, {{1}}},
  };

  for (auto &[numRows, ans] : tests) {
    assert(Solution().generate(numRows) == ans);
  }
}