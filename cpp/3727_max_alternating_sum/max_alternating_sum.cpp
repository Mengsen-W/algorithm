#include <algorithm>
#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    long long maxAlternatingSum(vector<int>& nums) {
        int n = nums.size();
        for (auto& num : nums) {
            num *= num;
        }
        ranges::sort(nums);
        return -accumulate(nums.begin(), nums.begin() + n / 2, 0LL) +
               accumulate(nums.begin() + n / 2, nums.end(), 0LL);
    }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{1, 2, 3}, 12},
      {{1, -1, 2, -2, 3, -3}, 16},
  };

  for (auto& [nums, expected] : tests) {
    assert(Solution().maxAlternatingSum(nums) == expected);
  }
}
