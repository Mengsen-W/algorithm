/*
 * @Date: 2022-09-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-17
 * @FilePath: /algorithm/1624_max_length_between_equal_characters/max_length_between_equal_characters.cpp
 */

#include <assert.h>

#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxLengthBetweenEqualCharacters(string s) {
    vector<int> firstIndex(26, -1);
    int maxLength = -1;
    for (int i = 0; i < s.size(); i++) {
      if (firstIndex[s[i] - 'a'] < 0) {
        firstIndex[s[i] - 'a'] = i;
      } else {
        maxLength = max(maxLength, i - firstIndex[s[i] - 'a'] - 1);
      }
    }
    return maxLength;
  }
};

int main() {
  assert(Solution().maxLengthBetweenEqualCharacters("aa") == 0);
  assert(Solution().maxLengthBetweenEqualCharacters("abca") == 2);
  assert(Solution().maxLengthBetweenEqualCharacters("cbzxy") == -1);
  assert(Solution().maxLengthBetweenEqualCharacters("cabbac") == 4);
}