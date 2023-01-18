/*
 * @Date: 2021-11-06 00:36:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-06 00:46:44
 */

#include <algorithm>
#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestSubsequence(vector<int> &arr, int difference) {
    int ans = 0;
    unordered_map<int, int> dp;
    for (int v : arr) {
      dp[v] = dp[v - difference] + 1;
      ans = max(ans, dp[v]);
    }
    return ans;
  }
};

int main() {
  {
    vector<int> arr{1, 2, 3, 4};
    int difference = 1;
    assert(Solution().longestSubsequence(arr, difference) == 4);
  }
  {
    vector<int> arr{1,3,5,7};
    int difference = 1;
    assert(Solution().longestSubsequence(arr, difference) == 1);
  }
  {
    vector<int> arr{1,5,7,8,5,3,4,2,1};
    int difference = -2;
    assert(Solution().longestSubsequence(arr, difference) == 4);
  }
}
