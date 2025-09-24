#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> sortArrayByParityII(vector<int>& nums) {
    int n = nums.size();
    int j = 1;
    for (int i = 0; i < n; i += 2) {
      if (nums[i] % 2 == 1) {
        while (nums[j] % 2 == 1) {
          j += 2;
        }
        swap(nums[i], nums[j]);
      }
    }
    return nums;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{4, 2, 5, 7}, {4, 5, 2, 7}},
      {{2, 3}, {2, 3}},
  };
  
  for (auto &[nums, ans] : tests) {
    assert(Solution().sortArrayByParityII(nums) == ans);
  }
}