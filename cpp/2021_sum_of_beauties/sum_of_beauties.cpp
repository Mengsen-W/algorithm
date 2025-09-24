#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int sumOfBeauties(vector<int>& nums) {
    int n = nums.size();
    vector<int> state(n);
    int pre_max = nums[0];
    for (int i = 1; i < n - 1; i++) {
      if (nums[i] > pre_max) {
        state[i] = 1;
        pre_max = nums[i];
      }
    }
    int suf_min = nums[n - 1];
    int res = 0;
    for (int i = n - 2; i > 0; i--) {
      if (state[i] && nums[i] < suf_min) {
        res += 2;
      } else if (nums[i - 1] < nums[i] && nums[i] < nums[i + 1]) {
        res += 1;
      }
      suf_min = min(suf_min, nums[i]);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3}, 2},
      {{2, 4, 6, 4}, 1},
      {{3, 2, 1}, 0},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().sumOfBeauties(nums) == ans);
  }
}