#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minRemoval(vector<int>& nums, int k) {
    int n = nums.size();
    sort(nums.begin(), nums.end());

    int ans = n, right = 0;
    for (int left = 0; left < n; ++left) {
      while (right < n && nums[right] <= static_cast<long long>(nums[left]) * k) {
        ++right;
      }
      ans = min(ans, n - (right - left));
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{2, 1, 5}, 2, 1},
      {{1, 6, 2, 9}, 3, 2},
      {{4, 6}, 2, 0},
  };

  for (auto [nums, k, ans] : tests) {
    assert(Solution().minRemoval(nums, k) == ans);
  }
}