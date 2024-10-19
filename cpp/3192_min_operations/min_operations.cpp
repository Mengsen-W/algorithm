#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int>& nums) {
    int operation = 0;
    for (int i = nums.size() - 2; i >= 0; i--) {
      if (nums[i] != nums[i + 1]) {
        operation++;
      }
    }
    return nums[0] == 1 ? operation : operation + 1;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{0, 1, 1, 0, 1}, 4},
      {{1, 0, 0, 0}, 1},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().minOperations(nums) == ans);
  }
}