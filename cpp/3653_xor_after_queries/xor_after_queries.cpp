#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
  static const int mod = 1e9 + 7;

 public:
  int xorAfterQueries(vector<int>& nums, vector<vector<int>>& queries) {
    // int n = nums.size();
    for (auto q : queries) {
      int l = q[0], r = q[1], k = q[2], v = q[3];
      for (int i = l; i <= r; i += k) {
        nums[i] = 1ll * nums[i] * v % mod;
      }
    }
    int res = 0;
    for (auto x : nums) {
      res ^= x;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>, int>> tests{
      {{1, 1, 1}, {{0, 2, 1, 4}}, 4},
      {{2, 3, 1, 5, 4}, {{1, 4, 2, 3}, {0, 2, 1, 2}}, 31},
  };

  for (auto& [nums, queries, ans] : tests) {
    assert(Solution().xorAfterQueries(nums, queries) == ans);
  }
}
