#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countAlternatingSubarrays(vector<int>& nums) {
    long long res = 0, cur = 0;
    int pre = -1;
    for (int a : nums) {
      cur = (pre != a) ? cur + 1 : 1;
      pre = a;
      res += cur;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{0, 1, 1, 1}, 5},
      {{1, 0, 1, 0}, 10},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().countAlternatingSubarrays(nums) == ans);
  }
}