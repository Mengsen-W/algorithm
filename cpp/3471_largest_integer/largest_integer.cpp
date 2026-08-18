#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int largestInteger(vector<int>& nums, int k) {
    int n = nums.size();
    if (n == k) {
      return *max_element(nums.begin(), nums.end());
    }
    int count[51] = {0};
    for (int x : nums) {
      count[x]++;
    }
    if (k == 1) {
      for (int i = 50; i >= 0; --i) {
        if (count[i] == 1) {
          return i;
        }
      }
      return -1;
    }
    int res = -1;
    if (count[nums[0]] == 1) {
      res = max(res, nums[0]);
    }
    if (count[nums.back()] == 1) {
      res = max(res, nums.back());
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{3, 9, 2, 1, 7}, 3, 7},
      {{3, 9, 7, 2, 1, 7}, 4, 3},
      {{0, 0}, 1, -1},
  };

  for (auto& [nums, k, expected] : tests) {
    assert(Solution().largestInteger(nums, k) == expected);
  }
}