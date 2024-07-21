#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumSum(vector<int>& arr) {
    int dp0 = arr[0], dp1 = 0, res = arr[0];
    for (int i = 1; i < arr.size(); i++) {
      dp1 = max(dp0, dp1 + arr[i]);
      dp0 = max(dp0, 0) + arr[i];
      res = max(res, max(dp0, dp1));
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {vector<int>{1, -2, 0, 3}, 4},
      {vector<int>{1, -2, -2, 3}, 3},
      {vector<int>{-1, -1, -1, -1}, -1},
  };
  for (auto& [arr, ans] : tests) {
    assert(Solution().maximumSum(arr) == ans);
  }
}