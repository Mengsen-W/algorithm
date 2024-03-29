/*
 * @Date: 2023-06-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-15
 * @FilePath: /algorithm/cpp/1777_can_make_pali_queries/can_make_pali_queries.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<bool> canMakePaliQueries(string s, vector<vector<int>>& queries) {
    int n = s.size();
    vector<int> count(n + 1);
    for (int i = 0; i < n; i++) {
      count[i + 1] = count[i] ^ (1 << (s[i] - 'a'));
    }
    vector<bool> res;
    for (auto& query : queries) {
      int l = query[0], r = query[1], k = query[2];
      int bits = 0, x = count[r + 1] ^ count[l];
      while (x > 0) {
        x &= x - 1;
        bits++;
      }
      res.push_back(bits <= k * 2 + 1);
    }
    return res;
  }
};

int main() {
  string s{"abcda"};
  vector<vector<int>> queries{{3, 3, 0}, {1, 2, 0}, {0, 3, 1}, {0, 3, 2}, {0, 4, 1}};
  vector<bool> ans{true, false, false, true, true};
  assert(Solution().canMakePaliQueries(s, queries) == ans);
}
