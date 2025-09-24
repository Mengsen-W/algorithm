#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maximumOr(vector<int>& nums, int k) {
    using ll = long long;
    ll or_sum = 0;
    ll multi_bits = 0;
    for (auto x : nums) {
      multi_bits |= x & or_sum;
      or_sum |= x;
    }

    ll res = 0;
    for (auto x : nums) {
      res = max(res, (or_sum ^ x) | (1ll * x << k) | multi_bits);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{12, 9}, 1, 30},
      {{8, 1, 2}, 2, 35},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().maximumOr(nums, k) == ans);
  }
}