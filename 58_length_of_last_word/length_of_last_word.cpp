/*
 * @Date: 2021-09-21 09:04:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-21 09:29:46
 */

#include <cassert>
#include <string>
using namespace std;

class Solution {
 public:
  int lengthOfLastWord(string s) {
    int index = s.size() - 1;

    while (s[index] == ' ') {
      index--;
    }
    int wordLength = 0;
    while (index >= 0 && s[index] != ' ') {
      wordLength++;
      index--;
    }

    return wordLength;
  }
};

int main() {
  {
    string s = "Hello World";
    assert(Solution().lengthOfLastWord(s) == 5);
  }
  {
    string s = "   fly me   to   the moon  ";
    assert(Solution().lengthOfLastWord(s) == 4);
  }
  {
    string s = "luffy is still joyboy";
    assert(Solution().lengthOfLastWord(s) == 6);
  }
}