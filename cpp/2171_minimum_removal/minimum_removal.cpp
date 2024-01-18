/*
 * @Date: 2024-01-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-18
 * @FilePath: /algorithm/cpp/2171_minimum_removal/minimum_removal.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long minimumRemoval(vector<int>& beans) {
    int n = beans.size();
    sort(beans.begin(), beans.end());
    long long total = accumulate(beans.begin(), beans.end(), 0LL);  // 豆子总数
    long long res = total;                                          // 最少需要移除的豆子数
    for (int i = 0; i < n; i++) {
      res = min(res, total - (long long)beans[i] * (n - i));
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{4, 1, 6, 5}, 4},
      {{2, 10, 3, 2}, 7},
  };

  for (auto& [beans, ans] : tests) {
    assert(Solution().minimumRemoval(beans) == ans);
  }
}