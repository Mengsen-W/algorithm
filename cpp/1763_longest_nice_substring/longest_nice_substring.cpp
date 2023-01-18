/*
 * @Date: 2022-02-01 04:02:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-01 05:17:26
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  string longestNiceSubstring(string s) {
    int maxPos = 0, maxLen = 0;
    auto check = [&](int typeNum) {
      vector<int> lowerCnt(26);
      vector<int> upperCnt(26);
      int cnt = 0;
      for (int l = 0, r = 0, total = 0; r < s.size(); ++r) {
        int idx = tolower(s[r]) - 'a';
        if (islower(s[r])) {
          ++lowerCnt[idx];
          if (lowerCnt[idx] == 1 && upperCnt[idx] > 0) {
            ++cnt;
          }
        } else {
          ++upperCnt[idx];
          if (upperCnt[idx] == 1 && lowerCnt[idx] > 0) {
            ++cnt;
          }
        }
        total += (lowerCnt[idx] + upperCnt[idx]) == 1 ? 1 : 0;

        while (total > typeNum) {
          idx = tolower(s[l]) - 'a';
          total -= (lowerCnt[idx] + upperCnt[idx]) == 1 ? 1 : 0;
          if (islower(s[l])) {
            --lowerCnt[idx];
            if (lowerCnt[idx] == 0 && upperCnt[idx] > 0) {
              --cnt;
            }
          } else {
            --upperCnt[idx];
            if (upperCnt[idx] == 0 && lowerCnt[idx] > 0) {
              --cnt;
            }
          }
          ++l;
        }
        if (cnt == typeNum && r - l + 1 > maxLen) {
          maxPos = l;
          maxLen = r - l + 1;
        }
      }
    };

    int mask = 0;
    for (char& ch : s) {
      mask |= 1 << (tolower(ch) - 'a');
    }
    int types = __builtin_popcount(mask);
    for (int i = 1; i <= types; ++i) {
      check(i);
    }
    return s.substr(maxPos, maxLen);
  }
};

int main() {
  assert(Solution().longestNiceSubstring("YazaAay") == "aAa");
  assert(Solution().longestNiceSubstring("Bb") == "Bb");
  assert(Solution().longestNiceSubstring("c") == "");
  assert(Solution().longestNiceSubstring("dDzeE") == "dD");
}