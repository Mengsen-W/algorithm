#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> resultsArray(vector<int>& nums, int k) {
    int n = nums.size();
    int cnt = 0;
    vector<int> ans(n - k + 1, -1);
    for (int i = 0; i < n; i++) {
      cnt = i == 0 || nums[i] - nums[i - 1] != 1 ? 1 : cnt + 1;
      if (cnt >= k) {
        ans[i - k + 1] = nums[i];
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<int>>> tests{
      {{1, 2, 3, 4, 3, 2, 5}, 3, {3, 4, -1, -1, -1}},
      {{2, 2, 2, 2, 2}, 4, {-1, -1}},
      {{3, 2, 3, 2, 3, 2}, 2, {-1, 3, -1, 3, -1}},
  };

  for (auto &[nums,k,ans] : tests) {
    assert(Solution().resultsArray(nums, k) == ans);
  }
}