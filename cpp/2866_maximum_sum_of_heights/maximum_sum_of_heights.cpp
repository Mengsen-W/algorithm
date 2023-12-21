/*
 * @Date: 2023-12-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-21
 * @FilePath: /algorithm/cpp/2866_maximum_sum_of_heights/maximum_sum_of_heights.cpp
 */

#include <cassert>
#include <stack>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maximumSumOfHeights(vector<int>& maxHeights) {
    int n = maxHeights.size();
    long long res = 0;
    vector<long long> prefix(n), suffix(n);
    stack<int> stack1, stack2;

    for (int i = 0; i < n; i++) {
      while (!stack1.empty() && maxHeights[i] < maxHeights[stack1.top()]) {
        stack1.pop();
      }
      if (stack1.empty()) {
        prefix[i] = (long long)(i + 1) * maxHeights[i];
      } else {
        prefix[i] = prefix[stack1.top()] + (long long)(i - stack1.top()) * maxHeights[i];
      }
      stack1.emplace(i);
    }
    for (int i = n - 1; i >= 0; i--) {
      while (!stack2.empty() && maxHeights[i] < maxHeights[stack2.top()]) {
        stack2.pop();
      }
      if (stack2.empty()) {
        suffix[i] = (long long)(n - i) * maxHeights[i];
      } else {
        suffix[i] = suffix[stack2.top()] + (long long)(stack2.top() - i) * maxHeights[i];
      }
      stack2.emplace(i);
      res = max(res, prefix[i] + suffix[i] - maxHeights[i]);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{5, 3, 4, 1, 1}, 13},
      {{6, 5, 3, 9, 2, 7}, 22},
      {{3, 2, 5, 5, 2, 3}, 18},
  };

  for (auto& [maxHeights, ans] : tests) {
    assert(Solution().maximumSumOfHeights(maxHeights) == ans);
  }
}
