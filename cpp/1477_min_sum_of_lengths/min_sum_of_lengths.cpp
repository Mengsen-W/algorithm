#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    int minSumOfLengths(vector<int>& arr, int target) {
        int n = arr.size();
        int ans = n + 1;
        int s = 0;
        vector<int> dp(n + 1, n);
        for (int l = 0, r = 0; r < n; r++) {
          s += arr[r];
          while (s > target) {
            s -= arr[l++];
          }
            dp[r + 1] = dp[r];
            if (s == target) {
                ans = min(ans, r - l + 1 + dp[l]);
                dp[r + 1] = min(dp[r], r - l + 1);
            }
        }
        return ans == n + 1 ? -1 : ans;
    }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{3, 2, 2, 4, 3}, 3, 2},
      {{7, 3, 4, 7}, 7, 2},
      {{3, 2, 2, 4, 3}, 6, -1},
      {{5, 5, 4, 4, 5}, 3, -1},
      {{3, 1, 1, 1, 5, 1, 2, 1}, 3, 3},
  };

  for (auto &[arr, target, ans] : tests) {
    assert(Solution().minSumOfLengths(arr, target) == ans);
  }
}