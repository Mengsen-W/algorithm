/*
 * @Date: 2021-11-13 01:15:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-13 01:23:02
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool detectCapitalUse(string word) {
    // 若第 1 个字母为小写，则需额外判断第 2 个字母是否为小写
    if (word.size() >= 2 && islower(word[0]) && isupper(word[1])) return false;

    // 无论第 1 个字母是否大写，其他字母必须与第 2 个字母的大小写相同
    int word_size = word.size();
    for (int i = 2; i < word_size; ++i)
      if (islower(word[i]) ^ islower(word[1])) return false;

    return true;
  }
};

int main() {
  assert(Solution().detectCapitalUse("USA"));
  assert(!Solution().detectCapitalUse("FlaG"));
}