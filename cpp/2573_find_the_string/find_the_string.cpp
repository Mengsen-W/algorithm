#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string findTheString(vector<vector<int>>& lcp) {
    int n = lcp.size();
    string word(n, '\0');
    char current = 'a';

    // 依次从 'a' 到 'z' 开始构造字符串
    for (int i = 0; i < n; i++) {
      if (word[i] == '\0') {
        if (current > 'z') {
          return "";
        }
        word[i] = current;
        for (int j = i + 1; j < n; j++) {
          if (lcp[i][j] > 0) {
            word[j] = word[i];
          }
        }
        current++;
      }
    }

    // 验证构造的字符串是否满足 LCP 矩阵要求
    for (int i = n - 1; i >= 0; i--) {
      for (int j = n - 1; j >= 0; j--) {
        if (word[i] != word[j]) {
          if (lcp[i][j]) {
            return "";
          }
        } else {
          if (i == n - 1 || j == n - 1) {
            if (lcp[i][j] != 1) {
              return "";
            }
          } else {
            if (lcp[i][j] != lcp[i + 1][j + 1] + 1) {
              return "";
            }
          }
        }
      }
    }

    return word;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, string>> tests{
      {{{4, 0, 2, 0}, {0, 3, 0, 1}, {2, 0, 2, 0}, {0, 1, 0, 1}}, "abab"},
      {{{4, 3, 2, 1}, {3, 3, 2, 1}, {2, 2, 2, 1}, {1, 1, 1, 1}}, "aaaa"},
      {{{4, 3, 2, 1}, {3, 3, 2, 1}, {2, 2, 2, 1}, {1, 1, 1, 3}}, ""},
  };

  for (auto [lcp, ans] : tests) {
    assert(Solution().findTheString(lcp) == ans);
  }
}