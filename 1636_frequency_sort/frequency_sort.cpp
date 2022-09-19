/*
 * @Date: 2022-09-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-19
 * @FilePath: /algorithm/1636_frequency_sort/frequency_sort.cpp
 */

#include <assert.h>

#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> frequencySort(vector<int>& nums) {
    unordered_map<int, int> cnt;
    for (int num : nums) {
      cnt[num]++;
    }
    sort(nums.begin(), nums.end(), [&](const int a, const int b) {
      if (cnt[a] != cnt[b]) {
        return cnt[a] < cnt[b];
      }
      return a > b;
    });
    return nums;
  }
};

int main() {
  {
    vector<int> nums{1, 1, 2, 2, 2, 3};
    vector<int> ans{3, 1, 1, 2, 2, 2};
    assert(Solution().frequencySort(nums) == ans);
  }

  {
    vector<int> nums{2, 3, 1, 3, 2};
    vector<int> ans{1, 3, 3, 2, 2};
    assert(Solution().frequencySort(nums) == ans);
  }

  {
    vector<int> nums{-1, 1, -6, 4, 5, -6, 1, 4, 1};
    vector<int> ans{5, -1, 4, 4, -6, -6, 1, 1, 1};
    assert(Solution().frequencySort(nums) == ans);
  }
}