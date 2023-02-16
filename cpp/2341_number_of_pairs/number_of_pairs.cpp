/*
 * @Date: 2023-02-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-16
 * @FilePath: /algorithm/cpp/2341_number_of_pairs/number_of_pairs.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> numberOfPairs(vector<int>& nums) {
    unordered_map<int, bool> cnt;
    int res = 0;
    for (int num : nums) {
      if (cnt.count(num)) {
        cnt[num] = !cnt[num];
      } else {
        cnt[num] = true;
      }
      if (!cnt[num]) {
        res++;
      }
    }
    return {res, (int)nums.size() - 2 * res};
  }
};

int main() {
  {
    vector<int> nums{1, 3, 2, 1, 3, 2, 2};
    vector<int> ans{3, 1};
    assert(Solution().numberOfPairs(nums) == ans);
  }

  {
    vector<int> nums{1, 1};
    vector<int> ans{1, 0};
    assert(Solution().numberOfPairs(nums) == ans);
  }

  {
    vector<int> nums{0};
    vector<int> ans{0, 1};
    assert(Solution().numberOfPairs(nums) == ans);
  }
}
