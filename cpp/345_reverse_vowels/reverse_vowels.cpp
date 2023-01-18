/*
 * @Date: 2021-08-19 09:44:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-19 09:54:01
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string reverseVowels(string s) {
    auto is_vowel = [vowels = "aeiouAEIOU"s](char ch) {
      return vowels.find(ch) != string::npos;
    };

    int n = s.size();
    int i = 0, j = n - 1;
    while (i < j) {
      while (i < n && !is_vowel(s[i])) ++i;

      while (j > 0 && !is_vowel(s[j])) --j;

      if (i < j) {
        swap(s[i], s[j]);
        ++i;
        --j;
      }
    }
    return s;
  }
};

int main() {
  {
    string s = "hello";
    string ans = "holle";
    assert(Solution{}.reverseVowels(s) == ans);
  }
  {
    string s = "leetcode";
    string ans = "leotcede";
    assert(Solution{}.reverseVowels(s) == ans);
  }
}