#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countMajoritySubarrays(vector<int>& nums, int target) {
    int n = nums.size();
    // 表示前缀和为 -n, -(n-1), ..., 0, 1, ..., n 的出现次数，下标偏移 n
    vector<int> pre((n * 2) + 1, 0);
    pre[n] = 1;
    int cnt = n;
    long long ans = 0;
    long long presum = 0;
    for (int i = 0; i < n; ++i) {
      if (nums[i] == target) {
        presum += pre[cnt];
        ++cnt;
        ++pre[cnt];
      } else {
        --cnt;
        presum -= pre[cnt];
        ++pre[cnt];
      }
      ans += presum;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{1, 2, 2, 3}, 2, 5},
      {{1, 1, 1, 1}, 1, 10},
      {{1, 2, 3}, 4, 0},
  };

  for (auto [nums, target, expected] : tests) {
    assert(Solution().countMajoritySubarrays(nums, target) == expected);
  }
}