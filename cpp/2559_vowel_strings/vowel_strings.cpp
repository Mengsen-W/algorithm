/*
 * @Date: 2023-06-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-02
 * @FilePath: /algorithm/cpp/2559_vowel_strings/vowel_strings.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> vowelStrings(vector<string>& words, vector<vector<int>>& queries) {
    int n = words.size();
    int prefixSums[n + 1];
    memset(prefixSums, 0, sizeof(prefixSums));
    for (int i = 0; i < n; i++) {
      int value = isVowelString(words[i]) ? 1 : 0;
      prefixSums[i + 1] = prefixSums[i] + value;
    }
    vector<int> ans(queries.size());
    for (int i = 0; i < queries.size(); i++) {
      int start = queries[i][0], end = queries[i][1];
      ans[i] = prefixSums[end + 1] - prefixSums[start];
    }
    return ans;
  }

  bool isVowelString(const string& word) { return isVowelLetter(word[0]) && isVowelLetter(word.back()); }

  bool isVowelLetter(char c) { return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'; }
};

int main() {
  {
    vector<string> words{"aba", "bcb", "ece", "aa", "e"};
    vector<vector<int>> queries{{0, 2}, {1, 4}, {1, 1}};
    vector<int> ans{2, 3, 0};
    assert(Solution().vowelStrings(words, queries) == ans);
  }

  {
    vector<string> words{"a", "e", "i"};
    vector<vector<int>> queries{{0, 2}, {0, 1}, {2, 2}};
    vector<int> ans{3, 2, 1};
    assert(Solution().vowelStrings(words, queries) == ans);
  }
}