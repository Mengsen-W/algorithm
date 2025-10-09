#include <cassert>
#include <tuple>
#include <utility>
#include <vector>
using namespace std;

class Solution {
 public:
  int triangularSum(vector<int>& nums) {
    while (nums.size() > 1) {
      vector<int> new_nums;
      for (size_t i = 0; i < nums.size() - 1; ++i) {
        new_nums.push_back((nums[i] + nums[i + 1]) % 10);
      }
      nums = std::move(new_nums);
    }
    return nums[0];
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 4, 5}, 8},
      {{5}, 5},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().triangularSum(nums) == ans);
  }
}
