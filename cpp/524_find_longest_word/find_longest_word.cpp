/*
 * @Date: 2021-09-14 08:52:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-14 10:45:21
 */

#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canFormByDeleting(string word, string str) {
    int word_i = 0, str_i = 0;
    int word_size = word.size();
    int str_size = str.size();

    while (word_i < word_size && str_i < str_size) {
      if (word[word_i] == str[str_i]) word_i++;

      str_i++;
    }

    return word_i == word_size;
  }

  string findLongestWord(string s, vector<string>& d) {
    string res = "";

    for (string& str : d)
      if (canFormByDeleting(str, s))
        if (str.size() > res.size() || (str.size() == res.size() && str < res))
          res = str;

    return res;
  }
};

int main() {
  {
    string s = "abpcplea";
    vector<string> dictionary{"ale", "apple", "monkey", "plea"};
    string ans = "apple";
    assert(Solution().findLongestWord(s, dictionary) == ans);
  }
  {
    string s = "abpcplea";
    vector<string> dictionary{"a", "b", "c"};
    string ans = "a";
    assert(Solution().findLongestWord(s, dictionary) == ans);
  }
  return 0;
}