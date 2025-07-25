#include <algorithm>
#include <cassert>
#include <numeric>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxSum(vector<int>& nums) {
    unordered_set<int> positiveNumsSet;
    for (int num : nums) {
      if (num > 0) {
        positiveNumsSet.emplace(num);
      }
    }
    if (positiveNumsSet.empty()) {
      return *max_element(nums.begin(), nums.end());
    }
    return accumulate(positiveNumsSet.begin(), positiveNumsSet.end(), 0);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 4, 5}, 15},
      {{1, 1, 0, 1, 1}, 1},
      {{1, 2, -1, -2, 1, 0, -1}, 3},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().maxSum(nums) == ans);
  }
}