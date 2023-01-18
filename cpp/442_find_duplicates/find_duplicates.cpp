/*
 * @Date: 2022-05-08 08:04:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-08 08:08:05
 * @FilePath: /algorithm/442_find_duplicates/find_duplicates.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findDuplicates(vector<int> nums) {
    int n = nums.size();
    vector<int> ans;
    for (int i = 0; i < n; ++i) {
      int x = abs(nums[i]);
      if (nums[x - 1] > 0) {
        nums[x - 1] = -nums[x - 1];
      } else {
        ans.push_back(x);
      }
    }
    return ans;
  }
};

int main() {
  assert((Solution().findDuplicates(vector<int>{4, 3, 2, 7, 8, 2, 3, 1}) == vector<int>{2, 3}));
  assert((Solution().findDuplicates(vector<int>{1, 1, 2}) == vector<int>{1}));
  assert((Solution().findDuplicates(vector<int>{1}) == vector<int>{}));
}