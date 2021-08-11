/*
 * @Date: 2021-08-11 14:38:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-11 14:43:41
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfArithmeticSlices(vector<int> &nums) {
    int ans = 0;
    int n = nums.size();
    vector<unordered_map<long long, int>> f(n);
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < i; ++j) {
        long long d = 1LL * nums[i] - nums[j];
        auto it = f[j].find(d);
        int cnt = it == f[j].end() ? 0 : it->second;
        ans += cnt;
        f[i][d] += cnt + 1;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{2, 4, 6, 8, 10};
    assert(Solution{}.numberOfArithmeticSlices(nums) == 7);
  }
  {
    vector<int> nums{7, 7, 7, 7, 7};
    assert(Solution{}.numberOfArithmeticSlices(nums) == 16);
  }
}