/*
 * @Date: 2022-11-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-25
 * @FilePath: /algorithm/809_expressive_words/expressive_words.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int expressiveWords(string s, vector<string>& words) {
    int ans = 0;
    for (const string& word : words) {
      if (expand(s, word)) {
        ++ans;
      }
    }
    return ans;
  }

 private:
  bool expand(const string& s, const string& t) {
    int i = 0, j = 0;
    while (i < s.size() && j < t.size()) {
      if (s[i] != t[j]) {
        return false;
      }
      char ch = s[i];
      int cnti = 0;
      while (i < s.size() && s[i] == ch) {
        ++cnti;
        ++i;
      }
      int cntj = 0;
      while (j < t.size() && t[j] == ch) {
        ++cntj;
        ++j;
      }
      if (cnti < cntj) {
        return false;
      }
      if (cnti != cntj && cnti < 3) {
        return false;
      }
    }
    return i == s.size() && j == t.size();
  }
};

int main() {
  string s{"heeellooo"};
  vector<string> words{"hello", "hi", "helo"};
  int ans = 1;
  assert(Solution().expressiveWords(s, words) == ans);
}