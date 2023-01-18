/*
 * @Date: 2022-05-14 09:03:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-14 10:12:14
 * @FilePath: /algorithm/691_min_stickers/min_stickers.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int minStickers(vector<string> stickers, string target) {
    int m = target.size();
    vector<int> dp(1 << m, -1);
    dp[0] = 0;
    function<int(int)> helper = [&](int mask) {
      if (dp[mask] != -1) {
        return dp[mask];
      }
      dp[mask] = m + 1;
      for (auto& sticker : stickers) {
        int left = mask;
        vector<int> cnt(26);
        for (char& c : sticker) {
          cnt[c - 'a']++;
        }
        for (int i = 0; i < m; i++) {
          if ((mask >> i & 1) && cnt[target[i] - 'a'] > 0) {
            cnt[target[i] - 'a']--;
            left ^= 1 << i;
          }
        }
        if (left < mask) {
          dp[mask] = min(dp[mask], helper(left) + 1);
        }
      }
      return dp[mask];
    };
    int res = helper((1 << m) - 1);
    return res > m ? -1 : res;
  }
};

int main() {
  assert(Solution().minStickers(vector<string>{"with", "example", "science"}, "thehat") == 3);
  assert(Solution().minStickers(vector<string>{"notice", "possible"}, "basicbasic") == -1);
}