/*
 * @Date: 2022-08-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-18
 * @FilePath: /algorithm/1224_max_equal_freq/max_equal_freq.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxEqualFreq(vector<int> nums) {
    unordered_map<int, int> freq, count;
    int res = 0, maxFreq = 0;
    for (int i = 0; i < nums.size(); i++) {
      if (count[nums[i]] > 0) {
        freq[count[nums[i]]]--;
      }
      count[nums[i]]++;
      maxFreq = max(maxFreq, count[nums[i]]);
      freq[count[nums[i]]]++;
      bool ok = maxFreq == 1 ||
                (freq[maxFreq] * maxFreq + freq[maxFreq - 1] * (maxFreq - 1) == i + 1 && freq[maxFreq] == 1) ||
                (freq[maxFreq] * maxFreq + 1 == i + 1 && freq[1] == 1);
      if (ok) {
        res = max(res, i + 1);
      }
    }
    return res;
  }
};

int main() {
  assert((Solution().maxEqualFreq({2, 2, 1, 1, 5, 3, 3, 5}) == 7));
  assert((Solution().maxEqualFreq({1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5}) == 13));
}
