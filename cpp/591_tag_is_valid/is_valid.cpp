/*
 * @Date: 2022-05-02 07:41:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-02 07:44:44
 * @FilePath: /algorithm/591_tag_is_valid/is_valid.cpp
 */

#include <cassert>
#include <stack>
#include <string>

using namespace std;

class Solution {
 public:
  bool isValid(string code) {
    int n = code.size();
    stack<string> tags;

    int i = 0;
    while (i < n) {
      if (code[i] == '<') {
        if (i == n - 1) {
          return false;
        }
        if (code[i + 1] == '/') {
          int j = code.find('>', i);
          if (j == string::npos) {
            return false;
          }
          string tagname = code.substr(i + 2, j - (i + 2));
          if (tags.empty() || tags.top() != tagname) {
            return false;
          }
          tags.pop();
          i = j + 1;
          if (tags.empty() && i != n) {
            return false;
          }
        } else if (code[i + 1] == '!') {
          if (tags.empty()) {
            return false;
          }
          string cdata = code.substr(i + 2, 7);
          if (cdata != "[CDATA[") {
            return false;
          }
          int j = code.find("]]>", i);
          if (j == string::npos) {
            return false;
          }
          i = j + 1;
        } else {
          int j = code.find('>', i);
          if (j == string::npos) {
            return false;
          }
          string tagname = code.substr(i + 1, j - (i + 1));
          if (tagname.size() < 1 || tagname.size() > 9) {
            return false;
          }
          if (!all_of(tagname.begin(), tagname.end(), [](unsigned char c) { return isupper(c); })) {
            return false;
          }
          tags.push(move(tagname));
          i = j + 1;
        }
      } else {
        if (tags.empty()) {
          return false;
        }
        ++i;
      }
    }

    return tags.empty();
  }
};

int main() {
  assert(Solution().isValid("<DIV>This is the first line <![CDATA[<div>]]></DIV>"));
  assert(Solution().isValid("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>"));
  assert(!Solution().isValid("<A>  <B> </A>   </B>"));
}