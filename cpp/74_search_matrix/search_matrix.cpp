/*
 * @Date: 2021-04-04 18:14:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-04 18:19:35
 */

#include <cassert>
#include <vector>

using namespace std;

bool search_matrix(vector<vector<int>>& matrix, int target) {
  int m = matrix.size(), n = matrix[0].size();
  int low = 0, high = m * n - 1;
  while (low <= high) {
    int mid = (high - low) / 2 + low;
    int x = matrix[mid / n][mid % n];
    if (x < target) {
      low = mid + 1;
    } else if (x > target) {
      high = mid - 1;
    } else {
      return true;
    }
  }
  return false;
}

int main() {
  vector<vector<int>> matrix{{1, 3, 5, 7}, {10, 11, 16, 20}, {23, 30, 34, 60}};
  assert(search_matrix(matrix, 3));
  matrix = {{1, 3, 5, 7}, {10, 11, 16, 20}, {23, 30, 34, 60}};
  assert(!search_matrix(matrix, 13));
}