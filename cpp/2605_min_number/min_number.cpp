/*
 * @Date: 2023-09-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-05
 * @FilePath: /algorithm/cpp/2605_min_number/min_number.cpp
 */

#include <algorithm>
#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int minNumber(vector<int>& nums1, vector<int>& nums2) {
    auto same = [&]() -> int {
      unordered_set<int> s(nums1.begin(), nums1.end());
      int x = 10;
      for (int num : nums2) {
        if (s.count(num)) {
          x = min(x, num);
        }
      }
      return x == 10 ? -1 : x;
    };

    if (int x = same(); x != -1) {
      return x;
    }

    int x = *min_element(nums1.begin(), nums1.end());
    int y = *min_element(nums2.begin(), nums2.end());
    return min(x * 10 + y, y * 10 + x);
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{4, 1, 3}, {5, 7}, 15},
      {{3, 5, 2, 6}, {3, 1, 7}, 3},
  };

  for (auto& [nums1, nums2, ans] : tests) {
    assert(Solution().minNumber(nums1, nums2) == ans);
  }
}