#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> divideArray(vector<int>& nums, int k) {
    sort(nums.begin(), nums.end());
    int n = nums.size();
    vector<vector<int>> res;
    for (int i = 0; i < n; i += 3) {
      if (nums[i + 2] - nums[i] > k) {
        return {};
      }
      res.emplace_back(nums.begin() + i, nums.begin() + i + 3);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<vector<int>>>> tests{
      {{1, 3, 4, 8, 7, 9, 3, 5, 1}, 2, {{1, 1, 3}, {3, 4, 5}, {7, 8, 9}}},
      {{2, 4, 2, 2, 5, 2}, 2, {}},
      {
          {4, 2, 9, 8, 2, 12, 7, 12, 10, 5, 8, 5, 5, 7, 9, 2, 5, 11},
          14,
          {{2, 2, 12}, {4, 8, 5}, {5, 9, 7}, {7, 8, 5}, {5, 9, 10}, {11, 12, 2}},
      },
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().divideArray(nums, k) == ans);
  }
}