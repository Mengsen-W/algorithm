/*
 * @Date: 2021-07-17 16:15:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-17 16:50:43
 */

#include <algorithm>
#include <cassert>
#include <iostream>

using namespace std;

class Solution {
 public:
  struct Status {
    int lSum, rSum, mSum, iSum;
  };

  Status pushUp(Status l, Status r) {
    int iSum = l.iSum + r.iSum;
    int lSum = max(l.lSum, l.iSum + r.lSum);
    int rSum = max(r.rSum, r.iSum + l.rSum);
    int mSum = max(max(l.mSum, r.mSum), l.rSum + r.lSum);
    return (Status){lSum, rSum, mSum, iSum};
  };

  Status get(vector<int>& a, int l, int r) {
    if (l == r) {
      return (Status){a[l], a[l], a[l], a[l]};
    }
    int m = (l + r) >> 1;
    Status lSub = get(a, l, m);
    Status rSub = get(a, m + 1, r);
    return pushUp(lSub, rSub);
  }

  int maxSubArray(vector<int>& nums) {
    return get(nums, 0, nums.size() - 1).mSum;
  }
};

int main() {
  // {
  //   vector<int> nums{-2, 1, -3, 4, -1, 2, 1, -5, 4};
  //   assert(Solution{}.maxSubArray(nums) == 6);
  // }
  {
    vector<int> nums{1, 2, -1, -2, 2, 1, -2, 1, 4, -5, 4};
    assert(Solution{}.maxSubArray(nums) == 6);
  }
}


/*
3,3,3,3
3,2,3,2
0,2,2,0
1,3,3,1
3,3,3,3
-1,1,1,-1
3,5,5,3
-1,4,4,-1
3,4,5,2
6,5,6,5
*/