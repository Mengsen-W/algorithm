/*
 * @Date: 2023-10-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-19
 * @FilePath: /algorithm/cpp/1726_tuple_same_product/tuple_same_product.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int tupleSameProduct(vector<int>& nums) {
    int n = nums.size();
    int ans = 0;
    unordered_map<int, int> cnt;
    for (int i = 0; i < n; i++) {
      for (int j = i + 1; j < n; j++) {
        cnt[nums[i] * nums[j]]++;
      }
    }
    for (auto& [k, v] : cnt) {
      ans += v * (v - 1) * 4;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 3, 4, 6}, 8},
      {{1, 2, 4, 5, 10}, 16},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().tupleSameProduct(nums) == ans);
  }
}