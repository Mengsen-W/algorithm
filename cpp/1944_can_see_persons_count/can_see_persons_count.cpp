/*
 * @Date: 2024-01-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-05
 * @FilePath: /algorithm/cpp/1944_can_see_persons_count/can_see_persons_count.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> canSeePersonsCount(vector<int>& heights) {
    int n = heights.size();
    vector<int> stack;
    vector<int> res(n, 0);

    for (int i = n - 1; i >= 0; i--) {
      int h = heights[i];
      while (!stack.empty() && stack.back() < h) {
        stack.pop_back();
        res[i] += 1;
      }
      if (!stack.empty()) {
        res[i] += 1;
      }
      stack.push_back(h);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{10, 6, 8, 5, 11, 9}, {3, 1, 2, 1, 1, 0}},
      {{5, 1, 2, 3, 10}, {4, 1, 1, 1, 0}},
  };

  for (auto& [heights, ans] : tests) {
    assert(Solution().canSeePersonsCount(heights) == ans);
  }
}