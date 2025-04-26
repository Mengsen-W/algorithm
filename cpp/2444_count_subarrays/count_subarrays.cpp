#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  using ll = long long;
  long long countSubarrays(vector<int>& nums, int minK, int maxK) {
    ll res = 0;
    int border = -1, last_min = -1, last_max = -1;
    for (int i = 0; i < nums.size(); i++) {
      if (nums[i] < minK || nums[i] > maxK) {
        last_max = -1;
        last_min = -1;
        border = i;
      }
      if (nums[i] == minK) {
        last_min = i;
      }
      if (nums[i] == maxK) {
        last_max = i;
      }
      if (last_min != -1 && last_max != -1) {
        res += min(last_min, last_max) - border;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, long long>> tests{
      {{1, 3, 5, 2, 7, 5}, 1, 5, 2},
      {{1, 1, 1, 1}, 1, 1,10},
  };

  for (auto &[nums, minK, maxK, ans] : tests) {
    assert(Solution().countSubarrays(nums, minK, maxK) == ans);
  }
}