/*
 * @Date: 2022-08-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-21
 * @FilePath: /algorithm/1455_is_prefix_of_word/is_prefix_of_word.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool isPrefix(const string &sentence, int start, int end, const string &searchWord) {
    for (int i = 0; i < searchWord.size(); i++) {
      if (start + i >= end || sentence[start + i] != searchWord[i]) {
        return false;
      }
    }
    return true;
  }

  int isPrefixOfWord(string sentence, string searchWord) {
    int n = sentence.size(), index = 1, start = 0, end = 0;
    while (start < n) {
      while (end < n && sentence[end] != ' ') {
        end++;
      }
      if (isPrefix(sentence, start, end, searchWord)) {
        return index;
      }

      index++;
      end++;
      start = end;
    }
    return -1;
  }
};

int main() {
  {
    string sentence{"i love eating burger"};
    string searchWord{"burg"};
    int ans = 4;
    assert(Solution().isPrefixOfWord(sentence, searchWord) == ans);
  }
  {
    string sentence{"this problem is an easy problem"};
    string searchWord{"pro"};
    int ans = 2;
    assert(Solution().isPrefixOfWord(sentence, searchWord) == ans);
  }
  {
    string sentence{"i am tired"};
    string searchWord{"you"};
    int ans = -1;
    assert(Solution().isPrefixOfWord(sentence, searchWord) == ans);
  }
}