/*
 * @Date: 2023-04-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-14
 * @FilePath: /algorithm/cpp/1023_camel_match/camel_match.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<bool> camelMatch(vector<string>& queries, string pattern) {
    int n = queries.size();
    vector<bool> res(n, true);
    for (int i = 0; i < n; i++) {
      int p = 0;
      for (auto c : queries[i]) {
        if (p < pattern.size() && pattern[p] == c) {
          p++;
        } else if (isupper(c)) {
          res[i] = false;
          break;
        }
      }
      if (p < pattern.size()) {
        res[i] = false;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<string> queries{"FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"};
    string pattern = "FB";
    vector<bool> ans{true, false, true, true, false};
    assert(Solution().camelMatch(queries, pattern) == ans);
  }

  {
    vector<string> queries{"FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"};
    string pattern = "FoBa";
    vector<bool> ans{true, false, true, false, false};
    assert(Solution().camelMatch(queries, pattern) == ans);
  }

  {
    vector<string> queries{"FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"};
    string pattern = "FoBaT";
    vector<bool> ans{false, true, false, false, false};
    assert(Solution().camelMatch(queries, pattern) == ans);
  }
}