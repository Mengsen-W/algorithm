#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countQuadruplets(vector<int>& nums) {
    int n = nums.size();
    vector<int> pre(n + 1);
    long long ans = 0;
    for (int j = 0; j < n; ++j) {
      int suf = 0;
      for (int k = n - 1; k > j; --k) {
        if (nums[j] > nums[k]) {
          ans += static_cast<long long>(pre[nums[k]]) * suf;
        } else {
          ++suf;
        }
      }
      for (int x = nums[j] + 1; x <= n; ++x) {
        ++pre[x];
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{1, 3, 2, 4, 5}, 2},
      {{1, 2, 3, 4}, 0},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().countQuadruplets(nums) == ans);
  }
}
