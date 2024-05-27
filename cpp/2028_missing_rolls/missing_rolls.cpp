/*
 * @Date: 2022-03-27 02:44:59
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-27
 * @FilePath: /algorithm/cpp/2028_missing_rolls/missing_rolls.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> missingRolls(vector<int>& rolls, int mean, int n) {
    int m = rolls.size();
    int sum = mean * (n + m);
    int missingSum = sum;
    for (int& roll : rolls) {
      missingSum -= roll;
    }
    if (missingSum < n || missingSum > 6 * n) {
      return {};
    }
    int quotient = missingSum / n, remainder = missingSum % n;
    vector<int> missing(n);
    for (int i = 0; i < n; i++) {
      missing[i] = quotient + (i < remainder ? 1 : 0);
    }
    return missing;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, vector<int>>> tests{
      {{3, 2, 4, 3}, 4, 2, {6, 6}},
      {{1, 5, 6}, 3, 4, {3, 2, 2, 2}},
      {{1, 2, 3, 4}, 5, 4, {}},
      {{1}, 3, 1, {5}},
  };

  for (auto& [rolls, mean, n, ans] : tests) {
    assert(Solution().missingRolls(rolls, mean, n) == ans);
  }
}