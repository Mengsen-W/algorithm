/*
 * @Date: 2023-04-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-14
 * @FilePath: /algorithm/cpp/2404_most_frequent_even/most_frequent_even.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int mostFrequentEven(vector<int>& nums) {
    unordered_map<int, int> count;
    for (auto x : nums) {
      if (x % 2 == 0) {
        count[x]++;
      }
    }
    int res = -1, ct = 0;
    for (auto& p : count) {
      if (res == -1 || p.second > ct || (p.second == ct && res > p.first)) {
        res = p.first;
        ct = p.second;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums{0, 1, 2, 2, 4, 4, 1};
    int ans = 2;
    assert(Solution().mostFrequentEven(nums) == ans);
  }

  {
    vector<int> nums{4, 4, 4, 9, 2, 4};
    int ans = 4;
    assert(Solution().mostFrequentEven(nums) == ans);
  }

  {
    vector<int> nums{29, 47, 21, 41, 13, 37, 25, 7};
    int ans = -1;
    assert(Solution().mostFrequentEven(nums) == ans);
  }
}