/*
 * @Date: 2022-01-30 00:59:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-30 01:08:41
 */

#include <cassert>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> uncommonFromSentences(string s1, string s2) {
    unordered_map<string, int> freq;

    auto insert = [&](const string& s) {
      stringstream ss(s);
      string word;
      while (ss >> word) {
        ++freq[move(word)];
      }
    };

    insert(s1);
    insert(s2);

    vector<string> ans;
    for (const auto& [word, occ] : freq) {
      if (occ == 1) {
        ans.push_back(word);
      }
    }
    return ans;
  }
};

int main() {
  assert((Solution().uncommonFromSentences("this apple is sweet",
                                           "this apple is sour") ==
          vector<string>{"sour", "sweet"}));
  assert((Solution().uncommonFromSentences("apple apple", "banana") ==
          vector<string>{"banana"}));
}