/*
 * @Date: 2021-06-08 08:38:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-08 08:55:16
 */

#include <cassert>
#include <numeric>
#include <vector>
using namespace std;

int lastStoneWeightII(vector<int> &stones) {
  int sum = accumulate(stones.begin(), stones.end(), 0);
  int m = sum / 2;
  vector<int> dp(m + 1);
  dp[0] = true;
  for (int weight : stones) {
    for (int j = m; j >= weight; --j) {
      dp[j] = dp[j] || dp[j - weight];
    }
  }
  for (int j = m;; --j) {
    if (dp[j]) {
      return sum - 2 * j;
    }
  }
}

int main() {
  {
    vector<int> stones{2, 7, 4, 1, 8, 1};
    assert(lastStoneWeightII(stones) == 1);
  }
  {
    vector<int> stones{31, 26, 33, 21, 40};
    assert(lastStoneWeightII(stones) == 5);
  }
  {
    vector<int> stones{1, 2};
    assert(lastStoneWeightII(stones) == 1);
  }
}
