/*
 * @Date: 2022-12-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-12
 * @FilePath: /algorithm/1781_beauty_sum/beauty_sum.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int beautySum(string s) {
    int res = 0;
    for (int i = 0; i < s.size(); i++) {
      vector<int> cnt(26);
      int maxFreq = 0;
      for (int j = i; j < s.size(); j++) {
        cnt[s[j] - 'a']++;
        maxFreq = max(maxFreq, cnt[s[j] - 'a']);
        int minFreq = s.size();
        for (int k = 0; k < 26; k++) {
          if (cnt[k] > 0) {
            minFreq = min(minFreq, cnt[k]);
          }
        }
        res += maxFreq - minFreq;
      }
    }
    return res;
  }
};

int main() {
  {
    string s{"aabcb"};
    int ans = 5;
    assert(Solution().beautySum(s) == ans);
  }

  {
    string s{"aabcbaa"};
    int ans = 17;
    assert(Solution().beautySum(s) == ans);
  }
}
