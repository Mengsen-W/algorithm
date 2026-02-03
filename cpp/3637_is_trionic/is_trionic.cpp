#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    bool isTrionic(vector<int>& nums) {
        if (nums[0] >= nums[1]) { // 一开始必须是递增的
            return false;
        }
        int cnt = 1;
        for (int i = 2; i < nums.size(); i++) {
            if (nums[i - 1] == nums[i]) {
                return false;
            }
            if ((nums[i - 2] < nums[i - 1]) != (nums[i - 1] < nums[i])) {
                cnt++;
            }
        }
        return cnt == 3; // 一定是增减增
    }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{1, 3, 5, 4, 2, 6}, true},
      {{2, 1, 3}, false},
  };

  for (auto&[nums, ans] : tests) {
    assert(Solution().isTrionic(nums) == ans);
  }
}