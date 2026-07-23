#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int uniqueXorTriplets(vector<int>& nums) {
    int n = nums.size();
    if (n <= 2) {
      return n;
    }
    int ans = 1;
    while (ans <= n) {
      ans <<= 1;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2}, 2},
      {{3, 1, 2}, 4},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().uniqueXorTriplets(nums) == expected);
  }
}
