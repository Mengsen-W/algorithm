/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-17 10:33:07
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-17 10:34:43
 */

#include <iostream>
#include <vector>

using namespace std;

vector<vector<int>> matrix_reshape(vector<vector<int>>& nums, int r, int c) {
  int m = nums.size();
  int n = nums[0].size();
  if (m * n != r * c) return nums;

  vector<vector<int>> ans(r, vector<int>(c));
  for (int i = 0; i < m * n; ++i) {
    ans[i / c][i % c] = nums[i / n][i % n];
  }
  return ans;
}

void print_vec(vector<vector<int>>& v) {
  for (int i = 0; i < v.size(); ++i) {
    for (int j = 0; j < v[0].size(); ++j) {
      cout << v[i][j] << ", ";
    }
    cout << endl;
  }
}

int main() {
  vector<vector<int>> nums{};
  nums = {{1, 2}, {3, 4}};
  vector<vector<int>> res = matrix_reshape(nums, 1, 4);
  print_vec(res);
  res = matrix_reshape(nums, 2, 4);
  print_vec(res);
}