/*
 * @Date: 2023-11-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-07
 * @FilePath: /algorithm/cpp/2586_vowel_strings/vowel_strings.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int vowelStrings(vector<string>& words, int left, int right) {
    unordered_set<char> vowels = {'a', 'e', 'i', 'o', 'u'};
    int ans = 0;
    for (int i = left; i <= right; ++i) {
      const string& word = words[i];
      if (vowels.count(word[0]) && vowels.count(word.back())) {
        ++ans;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<string>, int, int, int>> tests{
      {{"are", "amy", "u"}, 0, 2, 2},
      {{"hey", "aeo", "mu", "ooo", "artro"}, 1, 4, 3},
  };

  for (auto& [words, left, right, ans] : tests) {
    assert(Solution().vowelStrings(words, left, right) == ans);
  }
}