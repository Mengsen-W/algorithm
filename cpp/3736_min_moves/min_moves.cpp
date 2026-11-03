#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    int minMoves(vector<int>& nums) {
        int mx = nums[0];
        for (int x : nums) {
            mx = max(mx, x);
        }
        int ans = 0;
        for (int x : nums) {
            ans += mx - x;
        }
        return ans;
    }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 1, 3}, 3},
      {{4, 4, 5}, 2},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().minMoves(nums) == expected);
  }
}