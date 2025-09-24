#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int removeDuplicates(vector<int>& nums) {
    int n = nums.size();
    if (n <= 2) {
      return n;
    }
    int slow = 2, fast = 2;
    while (fast < n) {
      if (nums[slow - 2] != nums[fast]) {
        nums[slow] = nums[fast];
        ++slow;
      }
      ++fast;
    }
    return slow;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 1, 1, 2, 2, 3}, 5},
      {{0, 0, 1, 1, 1, 1, 2, 3, 3}, 7},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().removeDuplicates(nums) == ans);
  }
  return 0;
}