#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumOperations(vector<int>& nums) {
    vector<bool> seen(128);
    for (int i = nums.size() - 1; i >= 0; i--) {
      if (seen[nums[i]]) {
        return i / 3 + 1;
      }
      seen[nums[i]] = true;
    }
    return 0;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 4, 2, 3, 3, 5, 7}, 2},
      {{4, 5, 6, 4, 4}, 2},
      {{6, 7, 8, 9}, 0},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().minimumOperations(nums) == ans);
  }
}