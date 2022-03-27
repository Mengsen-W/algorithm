/*
 * @Date: 2022-03-27 02:44:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-27 02:53:14
 * @FilePath: /algorithm/2028_missing_rolls/missing_rolls.cpp
 */

#include <cassert>
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
  {
    vector<int> rolls{3, 2, 4, 3};
    int mean = 4;
    int n = 2;
    vector<int> ans{6, 6};
    assert(Solution().missingRolls(rolls, mean, n) == ans);
  }

  {
    vector<int> rolls{1, 5, 6};
    int mean = 3;
    int n = 4;
    vector<int> ans{3, 2, 2, 2};
    assert(Solution().missingRolls(rolls, mean, n) == ans);
  }

  {
    vector<int> rolls{1, 2, 3, 4};
    int mean = 5;
    int n = 4;
    vector<int> ans{};
    assert(Solution().missingRolls(rolls, mean, n) == ans);
  }

  {
    vector<int> rolls{1};
    int mean = 3;
    int n = 1;
    vector<int> ans{5};
    assert(Solution().missingRolls(rolls, mean, n) == ans);
  }
}