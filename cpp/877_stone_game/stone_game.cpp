/*
 * @Date: 2021-06-16 09:09:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-16 09:21:39
 */

#include <cassert>
#include <vector>
using namespace std;

bool stoneGame(vector<int>& piles) {
  int length = piles.size();
  auto dp = vector<int>(length);
  for (int i = 0; i < length; i++) {
    dp[i] = piles[i];
  }
  for (int i = length - 2; i >= 0; i--) {
    for (int j = i + 1; j < length; j++) {
      dp[j] = max(piles[i] - dp[j], piles[j] - dp[j - 1]);
    }
  }
  return dp[length - 1] > 0;
}

int main() {
  {
    vector<int> piles{5, 3, 4, 5};
    assert(stoneGame(piles));
  }
}