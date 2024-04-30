/*
 * @Date: 2024-04-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-30
 * @FilePath: /algorithm/cpp/2798_number_of_employees_who_met_target/number_of_employees_who_met_target.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfEmployeesWhoMetTarget(vector<int>& hours, int target) {
    int ans = 0;
    for (int i = 0; i < hours.size(); i++) {
      if (hours[i] >= target) {
        ans++;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{0, 1, 2, 3, 4}, 2, 3},
      {{5, 1, 4, 2, 2}, 6, 0},
  };

  for (auto& [hours, target, ans] : tests) {
    assert(Solution().numberOfEmployeesWhoMetTarget(hours, target) == ans);
  }
}