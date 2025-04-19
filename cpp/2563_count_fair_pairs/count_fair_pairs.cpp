#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countFairPairs(vector<int>& nums, int lower, int upper) {
    sort(nums.begin(), nums.end());
    long long ans = 0;
    int left = nums.size(), right = nums.size();
    for (int j = 0; j < nums.size(); ++j) {
      while (right && nums[right - 1] > upper - nums[j]) {
        right--;
      }
      while (left && nums[left - 1] >= lower - nums[j]) {
        left--;
      }
      ans += min(right, j) - min(left, j);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, long long>> tests{
      {{0, 1, 7, 4, 4, 5}, 3, 6, 6},
      {{1, 7, 9, 2, 5}, 11, 11, 1},
  };

  for (auto &[nums, lower, upper, ans] : tests) {
    assert(Solution().countFairPairs(nums, lower, upper) == ans);
  }

}