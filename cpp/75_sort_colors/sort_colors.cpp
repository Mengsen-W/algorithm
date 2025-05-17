#include <tuple>
#include <vector>
#include <cassert>

using namespace std;

class Solution {
 public:
  void sortColors(vector<int>& nums) {
    int n = nums.size();
    int p0 = 0, p2 = n - 1;
    for (int i = 0; i <= p2; ++i) {
      while (i <= p2 && nums[i] == 2) {
        swap(nums[i], nums[p2]);
        --p2;
      }
      if (nums[i] == 0) {
        swap(nums[i], nums[p0]);
        ++p0;
      }
    }
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests {
    {{2,0,2,1,1,0}, {0,0,1,1,2,2}},
    {{2,0,1}, {0,1,2}},
  };

  for (auto &[nums, ans] : tests) {
    Solution().sortColors(nums);
    assert(nums == ans);
  }
}