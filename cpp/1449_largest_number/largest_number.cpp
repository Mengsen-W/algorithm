/*
 * @Date: 2021-06-12 09:53:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-13 09:23:53
 */

#include <cassert>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

string largestNumber(vector<int> &cost, int target) {
  vector<int> dp(target + 1, INT_MIN);
  dp[0] = 0;
  for (int c : cost) {
    for (int j = c; j <= target; ++j) {
      dp[j] = max(dp[j], dp[j - c] + 1);
    }
  }
  if (dp[target] < 0) {
    return "0";
  }
  string ans;
  for (int i = 8, j = target; i >= 0; i--) {
    for (int c = cost[i]; j >= c && dp[j] == dp[j - c] + 1; j -= c) {
      ans += '1' + i;
    }
  }
  return ans;
}

int main() {
  {
    vector<int> cost{4, 3, 2, 5, 6, 7, 2, 5, 5};
    int target = 9;
    string res{"7772"};
    assert(largestNumber(cost, target) == res);
  }
  {
    vector<int> cost{2, 4, 6, 2, 4, 6, 4, 4, 4};
    int target = 5;
    string res{"0"};
    assert(largestNumber(cost, target) == res);
  }
  {
    vector<int> cost{7, 6, 5, 5, 5, 6, 8, 7, 8};
    int target = 12;
    string res{"85"};
    assert(largestNumber(cost, target) == res);
  }
  {
    vector<int> cost{6, 10, 15, 40, 40, 40, 40, 40, 40};
    int target = 47;
    string res{"32211"};
    assert(largestNumber(cost, target) == res);
  }
}