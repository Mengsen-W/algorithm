#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> buildArray(vector<int>& nums) {
    int n = nums.size();
    // 第一次遍历编码最终值
    for (int i = 0; i < n; ++i) {
      nums[i] += 1000 * (nums[nums[i]] % 1000);
    }
    // 第二次遍历修改为最终值
    for (int i = 0; i < n; ++i) {
      nums[i] /= 1000;
    }
    return nums;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{0, 2, 1, 5, 3, 4}, {0, 1, 2, 4, 5, 3}},
      {{5, 0, 1, 2, 3, 4}, {4, 5, 0, 1, 2, 3}},
  };

  for (auto &[nums, ans] : tests ) {
    assert(Solution().buildArray(nums) == ans);
  }
}