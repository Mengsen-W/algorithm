#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countSubarrays(vector<int>& nums, int k) {
    int mx = *max_element(nums.begin(), nums.end());
    long long ans = 0;
    int cnt = 0, left = 0;
    for (int x : nums) {
      if (x == mx) {
        cnt++;
      }
      while (cnt == k) {
        if (nums[left] == mx) {
          cnt--;
        }
        left++;
      }
      ans += left;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{1, 3, 2, 3, 3}, 2, 6},
      {{1, 4, 2, 1}, 3, 0},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().countSubarrays(nums, k) == ans);
  }
}