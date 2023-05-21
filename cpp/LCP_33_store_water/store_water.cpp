/*
 * @Date: 2023-05-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-21
 * @FilePath: /algorithm/cpp/LCP_33_store_water/store_water.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int storeWater(vector<int>& bucket, vector<int>& vat) {
    int n = bucket.size();
    int maxk = *max_element(vat.begin(), vat.end());
    if (maxk == 0) {
      return 0;
    }
    int res = INT_MAX;
    for (int k = 1; k <= maxk && k < res; ++k) {
      int t = 0;
      for (int i = 0; i < bucket.size(); ++i) {
        t += max(0, (vat[i] + k - 1) / k - bucket[i]);
      }
      res = min(res, t + k);
    }
    return res;
  }
};

int main() {
  {
    vector<int> bucket = {1, 3};
    vector<int> vat = {6, 8};
    int ans = 4;
    assert(Solution().storeWater(bucket, vat) == ans);
  }

  {
    vector<int> bucket = {9, 0, 1};
    vector<int> vat = {0, 2, 2};
    int ans = 3;
    assert(Solution().storeWater(bucket, vat) == ans);
  }
}