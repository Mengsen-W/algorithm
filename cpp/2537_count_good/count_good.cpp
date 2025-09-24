#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countGood(vector<int>& nums, int k) {
    int n = nums.size();
    int same = 0, right = -1;
    unordered_map<int, int> cnt;
    long long ans = 0;
    for (int left = 0; left < n; ++left) {
      while (same < k && right + 1 < n) {
        ++right;
        same += cnt[nums[right]];
        ++cnt[nums[right]];
      }
      if (same >= k) {
        ans += n - right;
      }
      --cnt[nums[left]];
      same -= cnt[nums[left]];
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{1, 1, 1, 1, 1}, 10, 1},
      {{3, 1, 4, 3, 2, 2, 4}, 2, 4},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().countGood(nums, k) == ans);
  }
}