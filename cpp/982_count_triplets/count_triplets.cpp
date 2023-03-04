/*
 * @Date: 2023-03-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-04
 * @FilePath: /algorithm/cpp/982_count_triplets/count_triplets.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int countTriplets(vector<int>& nums) {
    vector<int> cnt(1 << 16);
    for (int x : nums) {
      for (int y : nums) {
        ++cnt[x & y];
      }
    }
    int ans = 0;
    for (int x : nums) {
      x = x ^ 0xffff;
      for (int sub = x; sub; sub = (sub - 1) & x) {
        ans += cnt[sub];
      }
      ans += cnt[0];
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{2, 1, 3};
    int ans = 12;
    assert(Solution().countTriplets(nums) == ans);
  }

  {
    vector<int> nums{0,0,0};
    int ans = 27;
    assert(Solution().countTriplets(nums) == ans);
  }
}