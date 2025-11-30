#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countValidSelections(vector<int>& nums) {
    int n = nums.size();
    int ans = 0;
    int sum = accumulate(nums.begin(), nums.end(), 0);
    int leftSum = 0;
    int rightSum = sum;
    for (int i = 0; i < n; i++) {
      if (nums[i] == 0) {
        if (leftSum - rightSum >= 0 && leftSum - rightSum <= 1) {
          ans++;
        }
        if (rightSum - leftSum >= 0 && rightSum - leftSum <= 1) {
          ans++;
        }
      } else {
        leftSum += nums[i];
        rightSum -= nums[i];
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 0, 2, 0, 3}, 2},
      {{2, 3, 4, 0, 4, 1, 0}, 0},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().countValidSelections(nums) == ans);
  }
}
