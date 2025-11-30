#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool kLengthApart(vector<int>& nums, int k) {
    int n = nums.size();
    int pre = -1;
    for (int i = 0; i < n; ++i) {
      if (nums[i] == 1) {
        if (pre != -1 && i - pre - 1 < k) return false;
        pre = i;
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<int>, int, bool>> tests{
      {{1, 0, 0, 0, 1, 0, 0, 1}, 2, true},
      {{1, 0, 0, 1, 0, 1}, 2, false},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().kLengthApart(nums, k) == ans);
  }
}