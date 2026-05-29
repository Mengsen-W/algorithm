#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minElement(vector<int>& nums) {
    int ans = 37;
    for (int num : nums) {
      int dig = 0;
      while (num) {
        dig += num % 10;
        num /= 10;
      }
      ans = min(ans, dig);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{10, 12, 13, 14}, 1},
      {{1, 2, 3, 4}, 1},
      {{999, 19, 199}, 10},
  };

  for (auto [nums, expect] : tests) {
    assert(Solution().minElement(nums) == expect);
  }
}
