/*
 * @Date: 2023-07-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-08
 * @FilePath: /algorithm/cpp/167_two_sum/two_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> twoSum(vector<int>& numbers, int target) {
    int low = 0, high = numbers.size() - 1;
    while (low < high) {
      int sum = numbers[low] + numbers[high];
      if (sum == target) {
        return {low + 1, high + 1};
      } else if (sum < target) {
        ++low;
      } else {
        --high;
      }
    }
    return {-1, -1};
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<int>>> tests{
      {{2, 7, 11, 15}, 9, {1, 2}},
      {{2, 3, 4}, 6, {1, 3}},
      {{-1, 0}, -1, {1, 2}},
  };

  for (auto& [number, target, ans] : tests) {
    assert(Solution().twoSum(number, target) == ans);
  }
}