#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxOperations(vector<int>& nums) {
    int n = nums.size(), t = 0;
    for (int i = 1; i < n; i += 2) {
      if (nums[i] + nums[i - 1] != nums[1] + nums[0]) {
        break;
      }
      t++;
    }
    return t;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 2, 1, 4, 5}, 2},
      {{3, 2, 6, 1, 4}, 1},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().maxOperations(nums) == ans);
  }
}
