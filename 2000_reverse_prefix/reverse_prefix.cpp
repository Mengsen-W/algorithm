/*
 * @Date: 2022-02-02 00:53:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-02 00:59:31
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string reversePrefix(string word, char ch) {
    int index = word.find(ch);
    if (index != string::npos) {
      reverse(word.begin(), word.begin() + index + 1);
    }
    return word;
  }
};

int main() {
  assert(Solution().reversePrefix("abcdefd", 'd') == "dcbaefd");
  assert(Solution().reversePrefix("xyxzxe", 'z') == "zxyxxe");
  assert(Solution().reversePrefix("abcd", 'z') == "abcd");
}