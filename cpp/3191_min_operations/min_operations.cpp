#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int>& nums) {
    int n = nums.size();
    int ans = 0;

    for (int i = 0; i < n; i++) {
      if (nums[i] == 0) {
        if (i > n - 3) {
          return -1;
        }
        nums[i] ^= 1;
        nums[i + 1] ^= 1;
        nums[i + 2] ^= 1;
        ans++;
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{0, 1, 1, 1, 0, 0}, 3},
      {{0, 1, 1, 1}, -1},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().minOperations(nums) == ans);
  }
}