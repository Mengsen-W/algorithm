#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxDistance(vector<vector<int>>& arrays) {
    int res = 0;
    int n = arrays[0].size();
    int min_val = arrays[0][0];
    int max_val = arrays[0][arrays[0].size() - 1];
    for (int i = 1; i < arrays.size(); i++) {
      n = arrays[i].size();
      res = max(res, max(abs(arrays[i][n - 1] - min_val), abs(max_val - arrays[i][0])));
      min_val = min(min_val, arrays[i][0]);
      max_val = max(max_val, arrays[i][n - 1]);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 2, 3}, {4, 5}, {1, 2, 3}}, 4},
      {{{1}, {1}}, 0},
  };
  
  for (auto &[arrays, ans] : tests) {
    assert(Solution().maxDistance(arrays) == ans);
  }
}