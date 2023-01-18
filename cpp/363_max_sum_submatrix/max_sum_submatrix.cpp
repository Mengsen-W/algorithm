/*
 * @Date: 2021-04-22 09:52:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-22 09:58:37
 */

#include <cassert>
#include <set>
#include <vector>
using namespace std;

int max_sum_submatrix(vector<vector<int>> &matrix, int k) {
  int ans = INT_MIN;
  int m = matrix.size(), n = matrix[0].size();
  for (int i = 0; i < m; ++i) {  // 枚举上边界
    vector<int> sum(n);
    for (int j = i; j < m; ++j) {  // 枚举下边界
      for (int c = 0; c < n; ++c) {
        sum[c] += matrix[j][c];  // 更新每列的元素和
      }
      set<int> sumSet{0};
      int s = 0;
      for (int v : sum) {
        s += v;
        auto lb = sumSet.lower_bound(s - k);
        if (lb != sumSet.end()) {
          ans = max(ans, s - *lb);
        }
        sumSet.insert(s);
      }
    }
  }
  return ans;
}

int main() {
  {
    vector<vector<int>> nums{{1, 0, 1}, {0, -2, 3}};
    assert(max_sum_submatrix(nums, 2) == 2);
  }
  {
    vector<vector<int>> nums{{2, 2, -1}};
    assert(max_sum_submatrix(nums, 3) == 3);
  }
  return 0;
}