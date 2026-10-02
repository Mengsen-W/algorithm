#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<string> res;  // 记录答案
  vector<string> generateParenthesis(int n) {
    dfs(n, 0, 0, "");
    return res;
  }
  void dfs(int n, int lc, int rc, string str) {
    if (lc == n && rc == n)
      res.push_back(str);  // 递归边界
    else {
      if (lc < n) dfs(n, lc + 1, rc, str + "(");             // 拼接左括号
      if (rc < n && lc > rc) dfs(n, lc, rc + 1, str + ")");  // 拼接右括号
    }
  }
};

int main() {
  vector<tuple<int, vector<string>>> tests{
      {3, {"((()))", "(()())", "(())()", "()(())", "()()()"}},
      {1, {"()"}},
  };

  for (auto [n, expected] : tests) {
    assert(Solution().generateParenthesis(n) == expected);
  }
  return 0;
}