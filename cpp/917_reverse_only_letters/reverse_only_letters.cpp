/*
 * @Date: 2022-02-23 12:53:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-23 12:57:23
 * @FilePath: /algorithm/917_reverse_only_letters/reverse_only_letters.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string reverseOnlyLetters(string s) {
    int n = s.size();
    int left = 0, right = n - 1;
    while (true) {
      while (left < right && !isalpha(s[left])) left++;
      while (right > left && !isalpha(s[right])) right--;
      if (left >= right) break;
      swap(s[left], s[right]);
      left++;
      right--;
    }
    return s;
  }
};

int main() {
  assert(Solution().reverseOnlyLetters("ab-cd") == "dc-ba");
  assert(Solution().reverseOnlyLetters("a-bC-dEf-ghIj") == "j-Ih-gfE-dCba");
  assert(Solution().reverseOnlyLetters("Test1ng-Leet=code-Q!") ==
         "Qedo1ct-eeLg=ntse-T!");
}