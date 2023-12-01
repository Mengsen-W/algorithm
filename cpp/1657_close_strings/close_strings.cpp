/*
 * @Date: 2023-12-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-01
 * @FilePath: /algorithm/cpp/1657_close_strings/close_strings.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool closeStrings(string word1, string word2) {
    vector<int> count1(26), count2(26);
    for (char c : word1) {
      count1[c - 'a']++;
    }
    for (char c : word2) {
      count2[c - 'a']++;
    }
    for (int i = 0; i < 26; i++) {
      if ((count1[i] > 0 && count2[i] == 0) || (count1[i] == 0 && count2[i] > 0)) {
        return false;
      }
    }
    sort(count1.begin(), count1.end());
    sort(count2.begin(), count2.end());
    return count1 == count2;
  }
};

int main() {
  vector<tuple<string, string, bool>> tests{
      {"abc", "bca", true},
      {"a", "aa", false},
      {"cabbba", "abbccc", true},
  };

  for (auto& [word1, word2, ans] : tests) {
    assert(Solution().closeStrings(word1, word2) == ans);
  }
}