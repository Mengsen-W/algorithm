/*
 * @Date: 2021-07-23 18:50:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-23 18:54:45
 */

#include <cassert>
#include <vector>
using namespace std;

class Solution {
 public:
  bool isCovered(vector<vector<int>>& ranges, int left, int right) {
    vector<int> diff(52, 0);  // 差分数组
    for (auto&& range : ranges) {
      ++diff[range[0]];
      --diff[range[1] + 1];
    }
    // 前缀和
    int curr = 0;
    for (int i = 1; i <= 50; ++i) {
      curr += diff[i];
      if (i >= left && i <= right && curr <= 0) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  {
    vector<vector<int>> ranges = {{1, 2}, {3, 4}, {5, 6}};
    int left = 2;
    int right = 5;
    assert(Solution{}.isCovered(ranges, left, right));
  }
  {
    vector<vector<int>> ranges = {{10, 20}, {10, 20}};
    int left = 21;
    int right = 21;
    assert(!Solution{}.isCovered(ranges, left, right));
  }
}