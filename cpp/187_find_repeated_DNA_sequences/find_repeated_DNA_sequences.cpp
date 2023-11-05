/*
 * @Date: 2021-10-08 00:08:04
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-05
 */

#include <cassert>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
  const int L = 10;
  unordered_map<char, int> bin = {{'A', 0}, {'C', 1}, {'G', 2}, {'T', 3}};

 public:
  vector<string> findRepeatedDnaSequences(string s) {
    vector<string> ans;
    int n = s.length();
    if (n <= L) return ans;
    int x = 0;
    for (int i = 0; i < L - 1; ++i) x = (x << 2) | bin[s[i]];
    unordered_map<int, int> cnt;
    for (int i = 0; i <= n - L; ++i) {
      x = ((x << 2) | bin[s[i + L - 1]]) & ((1 << (L * 2)) - 1);
      if (++cnt[x] == 2) ans.push_back(s.substr(i, L));
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, vector<string>>> tests{
      {"AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT", {"AAAAACCCCC", "CCCCCAAAAA"}},
      {"AAAAAAAAAAAAA", {"AAAAAAAAAA"}},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().findRepeatedDnaSequences(s) == ans);
  }
}