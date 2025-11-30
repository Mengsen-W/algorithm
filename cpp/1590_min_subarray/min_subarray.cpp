#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int minSubarray(vector<int>& nums, int p) {
    int x = 0;
    for (auto num : nums) {
      x = (x + num) % p;
    }
    if (x == 0) {
      return 0;
    }
    unordered_map<int, int> index;
    int y = 0, res = nums.size();
    for (int i = 0; i < nums.size(); i++) {
      index[y] = i;  // f[i] mod p = y，因此哈希表记录 y 对应的下标为 i
      y = (y + nums[i]) % p;
      if (index.count((y - x + p) % p) > 0) {
        res = min(res, i - index[(y - x + p) % p] + 1);
      }
    }
    return res == nums.size() ? -1 : res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests = {
      {{3, 1, 4, 2}, 6, 1},
      {{6, 3, 5, 2}, 9, 2},
      {{1, 2, 3}, 3, 0},
      {{1, 2, 3}, 7, -1},
      {{1000000000, 1000000000, 1000000000}, 3, 0},
  };

  for (auto& [nums, p, expect] : tests) {
    assert(Solution().minSubarray(nums, p) == expect);
  }
}