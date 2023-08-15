/*
 * @Date: 2023-08-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-15
 * @FilePath: /algorithm/cpp/833_find_replace_string/find_replace_string.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  string findReplaceString(string s, vector<int>& indices, vector<string>& sources, vector<string>& targets) {
    int n = s.size(), m = indices.size();

    unordered_map<int, vector<int>> ops;
    for (int i = 0; i < m; ++i) {
      ops[indices[i]].push_back(i);
    }

    string ans;
    for (int i = 0; i < n;) {
      bool succeed = false;
      if (ops.count(i)) {
        for (int pt : ops[i]) {
          if (s.substr(i, sources[pt].size()) == sources[pt]) {
            succeed = true;
            ans += targets[pt];
            i += sources[pt].size();
            break;
          }
        }
      }
      if (!succeed) {
        ans += s[i];
        ++i;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, vector<int>, vector<string>, vector<string>, string>> tests{
      {"abcd", {0, 2}, {"a", "cd"}, {"eee", "ffff"}, "eeebffff"},
      {"abcd", {0, 2}, {"ab", "ec"}, {"eee", "ffff"}, "eeecd"},
  };

  for (auto& [s, indices, sources, targets, ans] : tests) {
    assert(Solution().findReplaceString(s, indices, sources, targets) == ans);
  }
}
