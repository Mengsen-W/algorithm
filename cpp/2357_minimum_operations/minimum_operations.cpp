/*
 * @Date: 2023-02-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-24
 * @FilePath: /algorithm/cpp/2357_minimum_operations/minimum_operations.cpp
 */

#include <cassert>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumOperations(vector<int>& nums) {
    unordered_set<int> hashSet;
    for (int num : nums) {
      if (num > 0) {
        hashSet.emplace(num);
      }
    }
    return hashSet.size();
  }
};

int main() {
  {
    vector<int> nums{1, 5, 0, 3, 5};
    int ans = 3;
    assert(Solution().minimumOperations(nums) == ans);
  }

  {
    vector<int> nums{0};
    int ans = 0;
    assert(Solution().minimumOperations(nums) == ans);
  }
}
