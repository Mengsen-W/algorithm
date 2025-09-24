#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findClosestNumber(vector<int>& nums) {
    int res = nums[0];       // 已遍历元素中绝对值最小且数值最大的元素
    int dis = abs(nums[0]);  // 已遍历元素的最小绝对值
    for (int num : nums) {
      if (abs(num) < dis) {
        dis = abs(num);
        res = num;
      } else if (abs(num) == dis) {
        res = max(res, num);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{-4, -2, 1, 4, 8}, 1},
      {{2, -1, 1}, 1},
  };
  
  for (auto &[nums, ans] : tests) {
    assert(Solution().findClosestNumber(nums) == ans);
  }
}