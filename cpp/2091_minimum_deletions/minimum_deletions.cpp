#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumDeletions(vector<int>& nums) {
    int n = nums.size();
    int minidx = min_element(nums.begin(), nums.end()) - nums.begin();
    int maxidx = max_element(nums.begin(), nums.end()) - nums.begin();
    int l = min(minidx, maxidx);                // 最值下标中的较小值
    int r = max(minidx, maxidx);                // 最值下标中的较大值
    return min({r + 1, n - l, l + 1 + n - r});  // 计算三种情况下删除次数的最小值
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 10, 7, 5, 4, 1, 8, 6}, 5},
      {{0,-4,19,1,8,-2,-3,5},3},
      {{101},1},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().minimumDeletions(nums) == ans);
  }
}