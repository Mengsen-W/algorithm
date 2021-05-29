/*
 * @Date: 2021-05-29 09:38:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-29 10:13:15
 */

#include <cassert>
#include <unordered_map>
#include <vector>
using namespace std;

int numSubmatrixSumTarget(vector<vector<int>> matrix, int target) {
  int m = matrix.size(), n = matrix[0].size(), ans = 0;
  //前缀和预处理
  for (int i = 0; i < m; ++i)
    for (int j = 1; j < n; ++j) matrix[i][j] += matrix[i][j - 1];

  for (int k = 0; k < n; ++k) {
    for (int j = k; j < n; ++j) {
      int sum = 0;
      //加入 {0,1} 以0，0为左上角的区间和等于target，属于边界情况
      unordered_map<int, int> mp = {{0, 1}};
      for (int i = 0; i < m; ++i) {
        //计算当前区间的区间和
        sum += (k == 0 ? matrix[i][j] : matrix[i][j] - matrix[i][k - 1]);
        if (mp.count(sum - target)) ans += mp[sum - target];
        ++mp[sum];
      }
    }
  }
  return ans;
}

int main() {
  assert(numSubmatrixSumTarget(
             vector<vector<int>>{{0, 1, 0}, {1, 1, 1}, {0, 1, 0}}, 0) == 4);
  assert(numSubmatrixSumTarget(vector<vector<int>>{{1, -1}, {-1, 1}}, 0) == 5);
  assert(numSubmatrixSumTarget(vector<vector<int>>{{904}}, 0) == 0);
}