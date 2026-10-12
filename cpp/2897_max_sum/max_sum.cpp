#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxSum(vector<int>& nums, int k) {
    const int MOD = 1e9 + 7;
    vector<int> cnt(30, 0);

    // 统计每个位上 1 的数量
    for (int num : nums) {
      for (int i = 0; i < 30; i++) {
        if (num & (1 << i)) {
          cnt[i]++;
        }
      }
    }

    long long res = 0;
    for (int i = 0; i < k; i++) {
      long long cur = 0;
      // 从高位到低位构造当前数字
      for (int j = 29; j >= 0; j--) {
        if (cnt[j] > 0) {
          cur |= (1LL << j);
          cnt[j]--;
        }
      }
      res = (res + cur * cur) % MOD;
    }

    return (int)res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{2, 6, 5, 8}, 2, 261},
      {{4, 5, 4, 7}, 3, 90},
  };

  for (auto [nums, k, expected] : tests) {
    assert(Solution().maxSum(nums, k) == expected);
  }
}