#include <cassert>
#include <cstdlib>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
  int getMinDistance(vector<int>& nums, int target, int start) {
    int res = nums.size();
    for (int i = 0; i < nums.size(); ++i) {
      if (nums[i] == target) {
        res = min(res, abs(i - start));
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, int>> tests{
    {{1, 2, 3, 4, 5}, 5, 3, 1},
    {{1}, 1, 0, 0},
    {{1, 1, 1, 1, 1, 1, 1, 1, 1, 1}, 1, 0, 0},
  };

  for (auto& [nums, target, start, ans] : tests) {
    assert(Solution().getMinDistance(nums, target, start) == ans);
  }
}
