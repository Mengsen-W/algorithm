/*
 * @Date: 2021-10-31 01:26:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-31 01:50:55
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> findWords(vector<string> words) {
    vector<string> ans;
    string rowIdx = "12210111011122000010020202";
    for (auto& word : words) {
      bool isValid = true;
      char idx = rowIdx[tolower(word[0]) - 'a'];
      int word_size = word.size();
      for (int i = 1; i < word_size; ++i)
        if (rowIdx[tolower(word[i]) - 'a'] != idx) {
          isValid = false;
          break;
        }
      if (isValid) ans.emplace_back(word);
    }
    return ans;
  }
};

int main() {
  assert(Solution().findWords({"Hello", "Alaska", "Dad", "Peace"}) ==
         vector<string>({"Alaska", "Dad"}));
  assert(Solution().findWords({"omk"}) == vector<string>{});
  assert(Solution().findWords({"adsdf", "sfd"}) ==
         vector<string>({"adsdf", "sfd"}));
}