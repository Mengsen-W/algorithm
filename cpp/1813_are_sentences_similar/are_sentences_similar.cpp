/*
 * @Date: 2023-01-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-16
 * @FilePath: /algorithm/1813_are_sentences_similar/are_sentences_similar.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string_view> split(const string& str, char target) {
    vector<string_view> res;
    string_view s(str);
    int pos = 0;
    while (pos < s.size()) {
      while (pos < s.size() && s[pos] == target) {
        pos++;
      }
      int start = pos;
      while (pos < s.size() && s[pos] != target) {
        pos++;
      }
      if (pos > start) {
        res.emplace_back(s.substr(start, pos - start));
      }
    }
    return res;
  }

  bool areSentencesSimilar(string sentence1, string sentence2) {
    vector<string_view> words1 = split(sentence1, ' ');
    vector<string_view> words2 = split(sentence2, ' ');
    int i = 0, j = 0;
    while (i < words1.size() && i < words2.size() && words1[i] == words2[i]) {
      i++;
    }
    while (j < words1.size() - i && j < words2.size() - i &&
           words1[words1.size() - j - 1] == (words2[words2.size() - j - 1])) {
      j++;
    }
    return i + j == min(words1.size(), words2.size());
  }
};

int main() {
  {
    string sentence1 = "My name is Haley";
    string sentence2 = "My Haley";
    assert(Solution().areSentencesSimilar(sentence1, sentence2));
  }

  {
    string sentence1 = "of";
    string sentence2 = "A lot of words";
    assert(!Solution().areSentencesSimilar(sentence1, sentence2));
  }

  {
    string sentence1 = "Eating right now";
    string sentence2 = "Eating";
    assert(Solution().areSentencesSimilar(sentence1, sentence2));
  }

  {
    string sentence1 = "Luky";
    string sentence2 = "Lucccky";
    assert(!Solution().areSentencesSimilar(sentence1, sentence2));
  }
}
