/*
 * @Date: 2024-04-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-08
 * @FilePath: /algorithm/cpp/2009_min_operations/min_operations.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int>& nums) {
    int n = nums.size();
    unordered_set<int> cnt(nums.begin(), nums.end());
    vector<int> sortedUniqueNums(cnt.begin(), cnt.end());
    sort(sortedUniqueNums.begin(), sortedUniqueNums.end());
    int res = n, j = 0;
    for (int i = 0; i < sortedUniqueNums.size(); i++) {
      int right = sortedUniqueNums[i] + n - 1;
      while (j < sortedUniqueNums.size() && sortedUniqueNums[j] <= right) {
        res = min(res, n - (j - i + 1));
        j++;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{4, 2, 5, 3}, 0},
      {{1, 2, 3, 5, 6}, 1},
      {{1, 10, 100, 1000}, 3},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().minOperations(nums) == ans);
  }
}