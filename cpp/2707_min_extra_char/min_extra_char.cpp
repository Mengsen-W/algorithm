/*
 * @Date: 2024-01-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-09
 * @FilePath: /algorithm/cpp/2707_min_extra_char/min_extra_char.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int minExtraChar(string s, vector<string>& dictionary) {
    int n = s.size();
    vector<int> d(n + 1, INT_MAX);
    unordered_map<string, int> mp;
    for (auto key : dictionary) {
      mp[key]++;
    }
    d[0] = 0;
    for (int i = 1; i <= n; i++) {
      d[i] = d[i - 1] + 1;
      for (int j = i - 1; j >= 0; j--) {
        if (mp.count(s.substr(j, i - j))) {
          d[i] = min(d[i], d[j]);
        }
      }
    }
    return d[n];
  }
};

int main() {
  vector<tuple<string, vector<string>, int>> tests{
      {"leetscode", {"leet", "code", "leetcode"}, 1},
      {"sayhelloworld", {"hello", "world"}, 3},
  };

  for (auto& [s, dictionary, ans] : tests) {
    assert(Solution().minExtraChar(s, dictionary) == ans);
  }
}