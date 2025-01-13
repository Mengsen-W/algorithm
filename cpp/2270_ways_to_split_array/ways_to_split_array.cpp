#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int waysToSplitArray(vector<int>& nums) {
    int n = nums.size();
    long long left = 0, right = accumulate(nums.begin(), nums.end(), 0LL);
    int ans = 0;
    for (int i = 0; i < n - 1; ++i) {
      left += nums[i];
      right -= nums[i];
      if (left >= right) {
        ++ans;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{10, 4, -8, 7}, 2},
      {{2, 3, 1, 0}, 2},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().waysToSplitArray(nums) == ans);
  }
}