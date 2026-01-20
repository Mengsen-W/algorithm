#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> minBitwiseArray(vector<int>& nums) {
    for (int& x : nums) {
      int res = -1;
      int d = 1;
      while ((x & d) != 0) {
        res = x - d;
        d <<= 1;
      }
      x = res;
    }
    return nums;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{2, 3, 5, 7}, {-1, 1, 4, 3}},
      {{11, 13, 31}, {9, 12, 15}},
  };

  for (auto& [nums, expected] : tests) {
    assert(Solution().minBitwiseArray(nums) == expected);
  }
}