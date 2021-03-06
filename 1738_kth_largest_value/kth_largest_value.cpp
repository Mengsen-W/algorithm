/*
 * @Date: 2021-05-19 08:47:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-19 09:07:24
 */

#include <algorithm>
#include <cassert>
#include <functional>
#include <vector>

using namespace std;

int kthLargestValue(const vector<vector<int>>& matrix, int k) {
  int m = matrix.size(), n = matrix[0].size();
  vector<vector<int>> pre(m + 1, vector<int>(n + 1));
  vector<int> results;
  for (int i = 1; i <= m; ++i) {
    for (int j = 1; j <= n; ++j) {
      pre[i][j] = pre[i - 1][j] ^ pre[i][j - 1] ^ pre[i - 1][j - 1] ^
                  matrix[i - 1][j - 1];
      results.push_back(pre[i][j]);
    }
  }

  nth_element(results.begin(), results.begin() + k - 1, results.end(),
              greater<int>());
  return results[k - 1];
}

int main() {
  assert(kthLargestValue(vector<vector<int>>{{5, 2}, {1, 6}}, 1) == 7);
  assert(kthLargestValue(vector<vector<int>>{{5, 2}, {1, 6}}, 2) == 5);
  assert(kthLargestValue(vector<vector<int>>{{5, 2}, {1, 6}}, 3) == 4);
  assert(kthLargestValue(vector<vector<int>>{{5, 2}, {1, 6}}, 4) == 0);
  return 0;
}