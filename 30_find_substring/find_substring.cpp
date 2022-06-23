/*
 * @Date: 2022-06-23
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-23
 * @FilePath: /algorithm/30_find_substring/find_substring.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findSubstring(string s, vector<string> words) {
    vector<int> res;
    int m = words.size(), n = words[0].size(), ls = s.size();
    for (int i = 0; i < n && i + m * n <= ls; ++i) {
      unordered_map<string, int> differ;
      for (int j = 0; j < m; ++j) {
        ++differ[s.substr(i + j * n, n)];
      }
      for (string &word : words) {
        if (--differ[word] == 0) {
          differ.erase(word);
        }
      }
      for (int start = i; start < ls - m * n + 1; start += n) {
        if (start != i) {
          string word = s.substr(start + (m - 1) * n, n);
          if (++differ[word] == 0) {
            differ.erase(word);
          }
          word = s.substr(start - n, n);
          if (--differ[word] == 0) {
            differ.erase(word);
          }
        }
        if (differ.empty()) {
          res.emplace_back(start);
        }
      }
    }
    return res;
  }
};

int main() {
  assert((Solution().findSubstring("barfoothefoobarman", vector<string>{"foo", "bar"}) == vector<int>{0, 9}));
  assert((Solution().findSubstring("wordgoodgoodgoodbestword", vector<string>{"word", "good", "best", "word"}) ==
          vector<int>{}));
  assert((Solution().findSubstring("barfoofoobarthefoobarman", vector<string>{"bar", "foo", "the"}) ==
          vector<int>{6, 9, 12}));
}