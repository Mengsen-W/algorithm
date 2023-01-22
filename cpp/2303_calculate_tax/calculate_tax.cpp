/*
 * @Date: 2023-01-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-23
 * @FilePath: /algorithm/cpp/2303_calculate_tax/calculate_tax.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  double calculateTax(vector<vector<int>>& brackets, int income) {
    double totalTax = 0;
    int lower = 0;
    for (auto& bracket : brackets) {
      int upper = bracket[0], percent = bracket[1];
      int tax = (min(income, upper) - lower) * percent;
      totalTax += tax;
      if (income <= upper) {
        break;
      }
      lower = upper;
    }
    return (double)totalTax / 100.0;
  }
};

int main() {
  {
    vector<vector<int>> brackets{{3, 50}, {7, 10}, {12, 25}};
    int income = 10;
    double ans = 2.65000000;
    assert(Solution().calculateTax(brackets, income) == ans);
  }

  {
    vector<vector<int>> brackets{{1, 0}, {4, 25}, {5, 50}};
    int income = 2;
    double ans = 0.25000000;
    assert(Solution().calculateTax(brackets, income) == ans);
  }

  {
    vector<vector<int>> brackets{{2, 50}};
    int income = 0;
    double ans = 0.000000;
    assert(Solution().calculateTax(brackets, income) == ans);
  }
}
