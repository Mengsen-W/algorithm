/*
 * @Date: 2023-03-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-19
 * @FilePath: /algorithm/cpp/1625_find_lex_smallest_string/find_lex_smallest_string.cpp
 */

#include <cassert>
#include <numeric>
#include <queue>
#include <string>
#include <unordered_set>

using namespace std;

class Solution {
 public:
  string findLexSmallestString(string s, int a, int b) {
    queue<string> q{{s}};
    unordered_set<string> vis{{s}};
    string ans = s;
    int n = s.size();
    while (!q.empty()) {
      s = q.front();
      q.pop();
      ans = min(ans, s);
      string t1 = s;
      for (int i = 1; i < n; i += 2) {
        t1[i] = (t1[i] - '0' + a) % 10 + '0';
      }
      string t2 = s.substr(n - b) + s.substr(0, n - b);
      for (auto& t : {t1, t2}) {
        if (!vis.count(t)) {
          vis.insert(t);
          q.emplace(t);
        }
      }
    }
    return ans;
  }
};

int main() {
  {
    string s{"5525"};
    int a = 9;
    int b = 2;
    string ans{"2050"};
    assert(Solution().findLexSmallestString(s, a, b) == ans);
  }

  {
    string s{"74"};
    int a = 5;
    int b = 1;
    string ans{"24"};
    assert(Solution().findLexSmallestString(s, a, b) == ans);
  }

  {
    string s{"0011"};
    int a = 4;
    int b = 2;
    string ans{"0011"};
    assert(Solution().findLexSmallestString(s, a, b) == ans);
  }

  {
    string s{"43987654"};
    int a = 7;
    int b = 3;
    string ans{"00553311"};
    assert(Solution().findLexSmallestString(s, a, b) == ans);
  }
}