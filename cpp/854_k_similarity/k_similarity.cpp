/*
 * @Date: 2022-09-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-21
 * @FilePath: /algorithm/854_k_similarity/k_similarity.cpp
 */

#include <assert.h>

#include <queue>
#include <string>
#include <unordered_set>

using namespace std;

class Solution {
 public:
  int kSimilarity(string s1, string s2) {
    int n = s1.size();
    queue<pair<string, int>> qu;
    unordered_set<string> visit;
    qu.emplace(s1, 0);
    visit.emplace(s1);
    for (int step = 0;; step++) {
      int sz = qu.size();
      for (int i = 0; i < sz; i++) {
        auto [cur, pos] = qu.front();
        qu.pop();
        if (cur == s2) return step;
        while (pos < n && cur[pos] == s2[pos]) pos++;
        for (int j = pos + 1; j < n; j++) {
          if (cur[j] != s2[j] && cur[j] == s2[pos]) {
            swap(cur[pos], cur[j]);
            if (!visit.count(cur)) {
              visit.emplace(cur);
              qu.emplace(cur, pos + 1);
            }
            swap(cur[pos], cur[j]);
          }
        }
      }
    }
  }
};

int main() {
  {
    string s1{"ab"}, s2{"ba"};
    int ans = 1;
    assert(Solution().kSimilarity(s1, s2) == ans);
  }

  {
    string s1{"abc"}, s2{"bca"};
    int ans = 2;
    assert(Solution().kSimilarity(s1, s2) == ans);
  }
}