/*
 * @Date: 2022-01-06 01:59:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-06 02:06:10
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  string simplifyPath(string path) {
    auto split = [](const string& s, char delim) -> vector<string> {
      vector<string> ans;
      string cur;
      for (char ch : s) {
        if (ch == delim) {
          ans.push_back(move(cur));
          cur.clear();
        } else {
          cur += ch;
        }
      }
      ans.push_back(move(cur));
      return ans;
    };

    vector<string> names = split(path, '/');
    vector<string> stack;
    for (string& name : names) {
      if (name == "..") {
        if (!stack.empty()) {
          stack.pop_back();
        }
      } else if (!name.empty() && name != ".") {
        stack.push_back(move(name));
      }
    }
    string ans;
    if (stack.empty()) {
      ans = "/";
    } else {
      for (string& name : stack) {
        ans += "/" + move(name);
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().simplifyPath("/home/") == "/home");
  assert(Solution().simplifyPath("/../") == "/");
  assert(Solution().simplifyPath("/home//foo/") == "/home/foo");
  assert(Solution().simplifyPath("/a/./b/../../c/") == "/c");
}