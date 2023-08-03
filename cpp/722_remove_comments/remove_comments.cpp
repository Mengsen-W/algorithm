/*
 * @Date: 2023-08-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-03
 * @FilePath: /algorithm/cpp/722_remove_comments/remove_comments.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> removeComments(vector<string>& source) {
    vector<string> ans;
    string t;
    bool blockComment = false;
    for (auto& s : source) {
      int m = s.size();
      for (int i = 0; i < m; ++i) {
        if (blockComment) {
          if (i + 1 < m && s[i] == '*' && s[i + 1] == '/') {
            blockComment = false;
            ++i;
          }
        } else {
          if (i + 1 < m && s[i] == '/' && s[i + 1] == '*') {
            blockComment = true;
            ++i;
          } else if (i + 1 < m && s[i] == '/' && s[i + 1] == '/') {
            break;
          } else {
            t.push_back(s[i]);
          }
        }
      }
      if (!blockComment && !t.empty()) {
        ans.emplace_back(t);
        t.clear();
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<string>, vector<string>>> tests{
      {{"/*Test program */", "int main()", "{ ", "  // variable declaration ", "int a, b, c;", "/* This is a test",
        "   multiline  ", "   comment for ", "   testing */", "a = b + c;", "}"},
       {"int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"}},
      {{"a/*comment", "line", "more_comment*/b"}, {"ab"}},
  };

  for (auto& [source, ans] : tests) {
    assert(Solution().removeComments(source) == ans);
  }
}