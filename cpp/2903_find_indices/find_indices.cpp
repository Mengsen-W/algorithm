/*
 * @Date: 2024-05-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-26
 * @FilePath: /algorithm/cpp/2903_find_indices/find_indices.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findIndices(vector<int>& nums, int indexDifference, int valueDifference) {
    int minIndex = 0, maxIndex = 0;
    for (int j = indexDifference; j < nums.size(); j++) {
      int i = j - indexDifference;
      if (nums[i] < nums[minIndex]) {
        minIndex = i;
      }
      if (nums[j] - nums[minIndex] >= valueDifference) {
        return {minIndex, j};
      }
      if (nums[i] > nums[maxIndex]) {
        maxIndex = i;
      }
      if (nums[maxIndex] - nums[j] >= valueDifference) {
        return {maxIndex, j};
      }
    }
    return {-1, -1};
  }
};

int main() {
  vector<tuple<vector<int>, int, int, vector<int>>> tests{
      {{5, 1, 4, 1}, 2, 4, {0, 3}},
      {{2, 1}, 0, 0, {0, 0}},
      {{1, 2, 3}, 2, 4, {-1, -1}},
  };

  for (auto& [nums, indexDifference, valueDifference, ans] : tests) {
    assert(Solution().findIndices(nums, indexDifference, valueDifference) == ans);
  }
}