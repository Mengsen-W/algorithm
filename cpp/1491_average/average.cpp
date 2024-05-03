/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-17 23:02:04
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-17 23:35:09
 */

#include <algorithm>
#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  double average(vector<int>& salary) {
    double maxValue = *max_element(salary.begin(), salary.end());
    double minValue = *min_element(salary.begin(), salary.end());
    double sum = accumulate(salary.begin(), salary.end(), -maxValue - minValue);
    return sum / int(salary.size() - 2);
  }
};

int main(void) {
  vector<tuple<vector<int>, double>> tests{
      {{4000, 3000, 1000, 2000}, 2500.00000},
      {{1000, 2000, 3000}, 2000.00000},
      {{6000, 5000, 4000, 3000, 2000, 1000}, 3500.00000},
      {{8000, 9000, 2000, 3000, 6000, 1000}, 4750.00000},
  };

  for (auto& [salary, ans] : tests) {
    assert(Solution().average(salary) == ans);
  }
}