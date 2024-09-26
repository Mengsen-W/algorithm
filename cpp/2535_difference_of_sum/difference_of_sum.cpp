#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int differenceOfSum(vector<int>& nums) {
    int ans = 0;
    for (int x : nums) {
      ans += x;  // 累加元素和
      while (x) {
        ans -= x % 10;  // 减去数位和
        x /= 10;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 15, 6, 3}, 9},
      {{1, 2, 3, 4}, 0},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().differenceOfSum(nums) == ans);
  }
}