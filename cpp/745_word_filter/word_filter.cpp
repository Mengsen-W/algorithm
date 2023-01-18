/*
 * @Date: 2022-07-14
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-14
 * @FilePath: /algorithm/745_word_filter/word_filter.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class WordFilter {
 private:
  unordered_map<string, int> dict;

 public:
  WordFilter(vector<string> words) {
    for (int i = 0; i < words.size(); i++) {
      int m = words[i].size();
      string word = words[i];
      for (int prefixLength = 1; prefixLength <= m; prefixLength++) {
        for (int suffixLength = 1; suffixLength <= m; suffixLength++) {
          string key = word.substr(0, prefixLength) + '#' + word.substr(m - suffixLength);
          dict[key] = i;
        }
      }
    }
  }

  int f(string pref, string suff) {
    string target = pref + '#' + suff;
    return dict.count(target) ? dict[target] : -1;
  }
};

int main() {
  WordFilter w{vector<string>{"apple"}};
  assert(w.f("a", "e") == 0);
}