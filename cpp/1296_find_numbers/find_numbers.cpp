#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findNumbers(vector<int>& nums) {
    return accumulate(nums.begin(), nums.end(), 0,
                      [](int ans, int num) { return ans + (static_cast<int>(log10(num) + 1) % 2 == 0); });
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{12, 345, 2, 6, 7896}, 2},
      {{555, 901, 482, 1771}, 1},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().findNumbers(nums) == ans);
  }
}
