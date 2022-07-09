/*
 * @Date: 2022-07-09
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-09
 * @FilePath: /algorithm/873_len_longest_fib_subseq/len_longest_fib_subseq.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int lenLongestFibSubseq(vector<int> arr) {
    unordered_map<int, int> indices;
    int n = arr.size();
    for (int i = 0; i < n; i++) {
      indices[arr[i]] = i;
    }
    vector<vector<int>> dp(n, vector<int>(n));
    int ans = 0;
    for (int i = 0; i < n; i++) {
      for (int j = i - 1; j >= 0 && arr[j] * 2 > arr[i]; j--) {
        int k = -1;
        if (indices.count(arr[i] - arr[j])) {
          k = indices[arr[i] - arr[j]];
        }
        if (k >= 0) {
          dp[j][i] = max(dp[k][j] + 1, 3);
        }
        ans = max(ans, dp[j][i]);
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().lenLongestFibSubseq(vector<int>{1, 2, 3, 4, 5, 6, 7, 8}) == 5);
  assert(Solution().lenLongestFibSubseq(vector<int>{1, 3, 7, 11, 12, 14, 18}) == 3);
}