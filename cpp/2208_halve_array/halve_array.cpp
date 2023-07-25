/*
 * @Date: 2023-07-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-25
 * @FilePath: /algorithm/cpp/2208_halve_array/halve_array.cpp
 */

#include <cassert>
#include <numeric>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int halveArray(vector<int>& nums) {
    priority_queue<double> pq(nums.begin(), nums.end());
    int res = 0;
    double sum = accumulate(nums.begin(), nums.end(), 0.0), sum2 = 0.0;
    while (sum2 < sum / 2) {
      double x = pq.top();
      pq.pop();
      sum2 += x / 2;
      pq.push(x / 2);
      res++;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{5, 19, 8, 1}, 3},
      {{3, 8, 20}, 3},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().halveArray(nums) == ans);
  }
}