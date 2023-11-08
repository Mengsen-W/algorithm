/*
 * @Date: 2023-11-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-08
 * @FilePath: /algorithm/cpp/2609_find_the_longest_balanced_substring/find_the_longest_balanced_substring.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findTheLongestBalancedSubstring(string s) {
    int res = 0, n = s.size();
    vector<int> count(2);
    for (int i = 0; i < n; i++) {
      if (s[i] == '1') {
        count[1]++;
        res = max(res, 2 * min(count[0], count[1]));
      } else if (i == 0 || s[i - 1] == '1') {
        count[0] = 1;
        count[1] = 0;
      } else {
        count[0]++;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"01000111", 6},
      {"00111", 4},
      {"111", 0},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().findTheLongestBalancedSubstring(s) == ans);
  }
}