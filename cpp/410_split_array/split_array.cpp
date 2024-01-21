/*
 * @Date: 2024-01-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-21
 * @FilePath: /algorithm/cpp/410_split_array/split_array.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool check(vector<int>& nums, int x, int m) {
    long long sum = 0;
    int cnt = 1;
    for (int i = 0; i < nums.size(); i++) {
      if (sum + nums[i] > x) {
        cnt++;
        sum = nums[i];
      } else {
        sum += nums[i];
      }
    }
    return cnt <= m;
  }

  int splitArray(vector<int>& nums, int m) {
    long long left = 0, right = 0;
    for (int i = 0; i < nums.size(); i++) {
      right += nums[i];
      if (left < nums[i]) {
        left = nums[i];
      }
    }
    while (left < right) {
      long long mid = (left + right) >> 1;
      if (check(nums, mid, m)) {
        right = mid;
      } else {
        left = mid + 1;
      }
    }
    return left;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{7, 2, 5, 10, 8}, 2, 18},
      {{1, 2, 3, 4, 5}, 2, 9},
      {{1, 4, 4}, 3, 4},
  };

  for (auto& [nums, m, ans] : tests) {
    assert(Solution().splitArray(nums, m) == ans);
  }
}