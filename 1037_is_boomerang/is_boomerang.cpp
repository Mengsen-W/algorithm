/*
 * @Date: 2022-06-08 09:41:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-08 09:45:01
 * @FilePath: /algorithm/1037_is_boomerang/is_boomerang.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isBoomerang(vector<vector<int>> points) {
    vector<int> v1 = {points[1][0] - points[0][0], points[1][1] - points[0][1]};
    vector<int> v2 = {points[2][0] - points[0][0], points[2][1] - points[0][1]};
    return v1[0] * v2[1] - v1[1] * v2[0] != 0;
  }
};

int main() {
  assert(Solution().isBoomerang({{1, 1}, {2, 3}, {3, 2}}) == true);
  assert(Solution().isBoomerang({{1, 1}, {2, 2}, {3, 3}}) == false);
}