/*
 * @Date: 2022-04-21 09:52:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-21 09:57:01
 * @FilePath: /algorithm/824_to_goat_latin/to_goat_latin.cpp
 */

#include <cassert>
#include <string>
#include <unordered_set>

using namespace std;

class Solution {
 public:
  string toGoatLatin(string sentence) {
    unordered_set<char> vowels = {'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'};

    int n = sentence.size();
    int i = 0, cnt = 1;
    string ans;

    while (i < n) {
      int j = i;
      while (j < n && sentence[j] != ' ') {
        ++j;
      }

      ++cnt;
      if (cnt != 2) {
        ans += ' ';
      }
      if (vowels.count(sentence[i])) {
        ans += sentence.substr(i, j - i) + 'm' + string(cnt, 'a');
      } else {
        ans += sentence.substr(i + 1, j - i - 1) + sentence[i] + 'm' + string(cnt, 'a');
      }

      i = j + 1;
    }

    return ans;
  }
};

int main() {
  assert(Solution().toGoatLatin("I speak Goat Latin") == "Imaa peaksmaaa oatGmaaaa atinLmaaaaa");
  assert(Solution().toGoatLatin("The quick brown fox jumped over the lazy dog") ==
         "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa");

  return 0;
}