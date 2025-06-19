#include <algorithm>
#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int partitionArray(vector<int>& nums, int k) {
    ranges::sort(nums);
    int ans = 0;
    int mn = INT_MIN / 2;  // 防止减法溢出
    for (int x : nums) {
      if (x - mn > k) {  // 必须分割
        ans++;
        mn = x;  // mn 是下一段的最小值
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{3, 6, 1, 2, 5}, 2, 2},
      {{1, 2, 3}, 1, 2},
      {{2, 2, 4, 5}, 0, 3},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().partitionArray(nums, k) == ans);
  }
}