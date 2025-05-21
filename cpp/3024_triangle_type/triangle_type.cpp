#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    string triangleType(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        if (nums[0] + nums[1] <= nums[2]) {
            return "none";
        } else if (nums[0] == nums[2]) {
            return "equilateral";
        } else if (nums[0] == nums[1] || nums[1] == nums[2]) {
            return "isosceles";
        } else {
            return "scalene";
        }
    }
};

int main() {
  vector<tuple<vector<int>, string>> tests {
    {{3,3,3}, "equilateral"},
    {{3,4,5}, "scalene"},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().triangleType(nums) == ans);
  }
}
