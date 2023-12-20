/*
 * @Date: 2023-12-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-20
 * @FilePath: /algorithm/cpp/2828_is_acronym/is_acronym.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isAcronym(vector<string>& words, string s) {
    if (s.size() != words.size()) {
      return false;
    }
    for (size_t i = 0; i < s.size(); i++) {
      if (words[i][0] != s[i]) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<string>, string, bool>> tests{
      {{"alice", "bob", "charlie"}, "abc", true},
      {{"an", "apple"}, "a", false},
      {{"never", "gonna", "give", "up", "on", "you"}, "ngguoy", true},
  };

  for (auto& [words, s, ans] : tests) {
    assert(Solution().isAcronym(words, s) == ans);
  }
}