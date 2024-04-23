/*
 * @Date: 2024-04-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-23
 * @FilePath: /algorithm/cpp/1052_max_satisfied/max_satisfied.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxSatisfied(vector<int>& customers, vector<int>& grumpy, int minutes) {
    int total = 0;
    int n = customers.size();
    for (int i = 0; i < n; i++) {
      if (grumpy[i] == 0) {
        total += customers[i];
      }
    }
    int increase = 0;
    for (int i = 0; i < minutes; i++) {
      increase += customers[i] * grumpy[i];
    }
    int maxIncrease = increase;
    for (int i = minutes; i < n; i++) {
      increase = increase - customers[i - minutes] * grumpy[i - minutes] + customers[i] * grumpy[i];
      maxIncrease = max(maxIncrease, increase);
    }
    return total + maxIncrease;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int, int>> tests{
      {{1, 0, 1, 2, 1, 1, 7, 5}, {0, 1, 0, 1, 0, 1, 0, 1}, 3, 16},
      {{1}, {0}, 1, 1},
  };

  for (auto& [customers, grumpy, minutes, ans] : tests) {
    assert(Solution().maxSatisfied(customers, grumpy, minutes) == ans);
  }
}