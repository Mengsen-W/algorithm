/*
 * @Date: 2023-12-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-03
 * @FilePath: /algorithm/cpp/1423_max_score/max_score.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxScore(vector<int>& cardPoints, int k) {
    int n = cardPoints.size();
    // 滑动窗口大小为 n-k
    int windowSize = n - k;
    // 选前 n-k 个作为初始值
    int sum = accumulate(cardPoints.begin(), cardPoints.begin() + windowSize, 0);
    int minSum = sum;
    for (int i = windowSize; i < n; ++i) {
      // 滑动窗口每向右移动一格，增加从右侧进入窗口的元素值，并减少从左侧离开窗口的元素值
      sum += cardPoints[i] - cardPoints[i - windowSize];
      minSum = min(minSum, sum);
    }
    return accumulate(cardPoints.begin(), cardPoints.end(), 0) - minSum;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{{{1, 2, 3, 4, 5, 6, 1}, 3, 12},
                                             {{2, 2, 2}, 2, 4},
                                             {{9, 7, 7, 9, 7, 7, 9}, 7, 55},
                                             {{1, 1000, 1}, 1, 1},
                                             {{1, 79, 80, 1, 1, 1, 200, 1}, 3, 202}};

  for (auto& [cardPoints, k, ans] : tests) {
    assert(Solution().maxScore(cardPoints, k) == ans);
  }
}