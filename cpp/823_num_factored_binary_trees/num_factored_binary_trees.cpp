/*
 * @Date: 2023-08-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-29
 * @FilePath: /algorithm/cpp/823_num_factored_binary_trees/num_factored_binary_trees.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numFactoredBinaryTrees(vector<int>& arr) {
    sort(arr.begin(), arr.end());
    int n = arr.size();
    vector<long long> dp(n);
    long long res = 0, mod = 1e9 + 7;
    for (int i = 0; i < n; i++) {
      dp[i] = 1;
      for (int left = 0, right = i - 1; left <= right; left++) {
        while (right >= left && (long long)arr[left] * arr[right] > arr[i]) {
          right--;
        }
        if (right >= left && (long long)arr[left] * arr[right] == arr[i]) {
          if (right != left) {
            dp[i] = (dp[i] + dp[left] * dp[right] * 2) % mod;
          } else {
            dp[i] = (dp[i] + dp[left] * dp[right]) % mod;
          }
        }
      }
      res = (res + dp[i]) % mod;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 4}, 3},
      {{2, 4, 5, 10}, 7},
  };

  for (auto& [arr, ans] : tests) {
    assert(Solution().numFactoredBinaryTrees(arr) == ans);
  }
}