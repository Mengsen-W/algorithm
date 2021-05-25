/*
 * @Date: 2021-05-25 09:34:13
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-25 19:38:24
 */

#include <algorithm>
#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

static constexpr int N = 1024;  // 2^10
int minChanges(const vector<int>& nums, int k) {
  int n = nums.size();
  vector<int> group_amount(k);
  vector<unordered_map<int, int>> group_record(k);
  for (int i = 0; i < n; i++) {
    group_amount[i % k]++;
    group_record[i % k][nums[i]]++;
  }

  vector<vector<int>> dp(k, vector<int>(N));
  for (int j = 0; j < N; j++) dp[0][j] = group_amount[0] - group_record[0][j];

  for (int i = 1; i < k; i++) {
    int upper_limit =
        *min_element(dp[i - 1].begin(), dp[i - 1].end()) + group_amount[i];
    fill(dp[i].begin(), dp[i].end(), upper_limit);

    for (auto [num, amount] : group_record[i % k])
      for (int j = 0; j < N; j++)
        dp[i][j ^ num] =
            min(dp[i][j ^ num], dp[i - 1][j] + group_amount[i] - amount);
  }
  return dp[k - 1][0];
}

int main() {
  assert(minChanges(vector<int>{1, 2, 0, 3, 0}, 1) == 3);
  assert(minChanges(vector<int>{3, 4, 5, 2, 1, 7, 3, 4, 7}, 3) == 3);
  assert(minChanges(vector<int>{1, 2, 4, 1, 2, 5, 1, 2, 6}, 3) == 3);
}