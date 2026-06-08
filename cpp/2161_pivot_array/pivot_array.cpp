#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> pivotArray(vector<int>& nums, int pivot) {
    int n = nums.size();
    vector<int> ans(n, pivot);
    int left = 0, right = n - 1;
    for (int i = 0; i < n; ++i) {
      if (nums[i] < pivot) {
        ans[left] = nums[i];
        ++left;
      } else if (nums[i] > pivot) {
        ans[right] = nums[i];
        --right;
      }
    }
    reverse(ans.begin() + right + 1, ans.end());
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<int>>> tests{
      {{9, 12, 5, 10, 14, 3, 10}, 10, {9, 5, 3, 10, 10, 12, 14}},
      {{-3, 4, 3, 2}, 2, {-3, 2, 4, 3}},
  };

  for (auto& [nums, pivot, expect] : tests) {
    assert(Solution().pivotArray(nums, pivot) == expect);
  }
}