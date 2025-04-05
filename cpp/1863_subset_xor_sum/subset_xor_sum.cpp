#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int subsetXORSum(vector<int>& nums) {
    int res = 0;
    int n = nums.size();
    for (auto num : nums) {
      res |= num;
    }
    return res << (n - 1);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 3}, 6},
      {{5, 1, 6}, 28},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().subsetXORSum(nums) == ans);
  }
}