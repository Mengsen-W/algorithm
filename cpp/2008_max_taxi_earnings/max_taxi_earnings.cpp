/*
 * @Date: 2023-12-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-08
 * @FilePath: /algorithm/cpp/2008_max_taxi_earnings/max_taxi_earnings.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maxTaxiEarnings(int n, vector<vector<int>> &rides) {
    vector<long long> dp(n + 1);
    unordered_map<int, vector<vector<int>>> rideMap;
    for (const auto &ride : rides) {
      rideMap[ride[1]].push_back(ride);
    }
    for (int i = 1; i <= n; i++) {
      dp[i] = dp[i - 1];
      for (const auto &ride : rideMap[i]) {
        dp[i] = max(dp[i], dp[ride[0]] + ride[1] - ride[0] + ride[2]);
      }
    }
    return dp[n];
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, long long>> tests{
      {5, {{2, 5, 4}, {1, 5, 1}}, 7},
      {20, {{1, 6, 1}, {3, 10, 2}, {10, 12, 3}, {11, 12, 2}, {12, 15, 2}, {13, 18, 1}}, 20},
  };

  for (auto &[n, rides, ans] : tests) {
    assert(Solution().maxTaxiEarnings(n, rides) == ans);
  }
}