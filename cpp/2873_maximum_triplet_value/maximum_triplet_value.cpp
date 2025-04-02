#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maximumTripletValue(vector<int>& nums) {
    int n = nums.size();
    long long res = 0, imax = 0, dmax = 0;
    for (int k = 0; k < n; k++) {
      res = max(res, dmax * nums[k]);
      dmax = max(dmax, imax - nums[k]);
      imax = max(imax, static_cast<long long>(nums[k]));
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{12, 6, 1, 2, 7}, 77},
      {{1, 10, 3, 4, 19}, 133},
      {{1, 2, 3}, 0},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().maximumTripletValue(nums) == ans);
  }
}