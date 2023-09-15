/*
 * @Date: 2023-09-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-15
 * @FilePath: /algorithm/cpp/LCP_50_give_gem/give_gem.cpp
 */

#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int giveGem(vector<int>& gem, vector<vector<int>>& operations) {
    for (auto& operation : operations) {
      int x = operation[0], y = operation[1];
      int number = gem[x] / 2;
      gem[x] -= number;
      gem[y] += number;
    }
    int mn = *min_element(gem.begin(), gem.end());
    int mx = *max_element(gem.begin(), gem.end());
    return mx - mn;
  }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>, int>> test_cases{
      {{3, 1, 2}, {{0, 2}, {2, 1}, {2, 0}}, 2},
      {{100, 0, 50, 100}, {{0, 2}, {0, 1}, {3, 0}, {3, 0}}, 75},
      {{0, 0, 0, 0}, {{1, 2}, {3, 1}, {1, 2}}, 0},
  };
}