/*
 * @Date: 2022-09-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-07
 * @FilePath: /algorithm/1592_reorder_spaces/reorder_spaces.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string_view> split(const string_view &str, char trim) {
    int n = str.size();
    vector<string_view> res;
    int pos = 0;
    while (pos < n) {
      while (pos < n && str[pos] == trim) {
        pos++;
      }
      if (pos < n) {
        int curr = pos;
        while (pos < n && str[pos] != trim) {
          pos++;
        }
        res.emplace_back(str.substr(curr, pos - curr));
      }
    }
    return res;
  }

  string reorderSpaces(string text) {
    int length = text.size();
    vector<string_view> words = split(text, ' ');
    int cntSpace = length;
    int wordCount = 0;
    for (auto &word : words) {
      if (word.size() > 0) {
        cntSpace -= word.size();
        wordCount++;
      }
    }

    string ans;
    if (words.size() == 1) {
      ans.append(words[0]);
      for (int i = 0; i < cntSpace; i++) {
        ans.push_back(' ');
      }
      return ans;
    }
    int perSpace = cntSpace / (wordCount - 1);
    int restSpace = cntSpace % (wordCount - 1);
    for (int i = 0; i < words.size(); i++) {
      if (words[i].size() == 0) {
        continue;
      }
      if (ans.size() > 0) {
        for (int j = 0; j < perSpace; j++) {
          ans.push_back(' ');
        }
      }
      ans.append(words[i]);
    }
    for (int i = 0; i < restSpace; i++) {
      ans.push_back(' ');
    }
    return ans;
  }
};

int main() {
  {
    string text{"  this   is  a sentence "};
    string ans{"this   is   a   sentence"};
    assert(Solution().reorderSpaces(text) == ans);
  }

  {
    string text{" practice   makes   perfect"};
    string ans{"practice   makes   perfect "};
    assert(Solution().reorderSpaces(text) == ans);
  }

  {
    string text{"hello   world"};
    string ans{"hello   world"};
    assert(Solution().reorderSpaces(text) == ans);
  }

  {
    string text{"  walks  udp package   into  bar a"};
    string ans{"walks  udp  package  into  bar  a "};
    assert(Solution().reorderSpaces(text) == ans);
  }

  {
    string text{"a"};
    string ans{"a"};
    assert(Solution().reorderSpaces(text) == ans);
  }
}