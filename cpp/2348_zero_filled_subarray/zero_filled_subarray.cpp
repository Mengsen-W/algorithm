#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long zeroFilledSubarray(vector<int>& nums) {
    long long ans = 0;
    int cnt0 = 0;
    for (int x : nums) {
      if (x) {
        cnt0 = 0;
      } else {
        cnt0++;  // 右端点为 i 的全 0 子数组比右端点为 i-1 的全 0 子数组多一个
        ans += cnt0;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{1, 3, 0, 0, 2, 0, 0, 4}, 6},
      {{0, 0, 0, 2, 0, 0}, 9},
      {{2, 10, 2019}, 0},
  };

  for (auto [nums, expect] : tests) {
    assert(Solution().zeroFilledSubarray(nums) == expect);
  }
}