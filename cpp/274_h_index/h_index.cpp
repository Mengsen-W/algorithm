/*
 * @Date: 2021-07-12 08:24:39
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-29
 */

#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int hIndex(vector<int>& citations) {
    int n = citations.size(), tot = 0;
    vector<int> counter(n + 1);
    for (int i = 0; i < n; i++) {
      if (citations[i] >= n) {
        counter[n]++;
      } else {
        counter[citations[i]]++;
      }
    }
    for (int i = n; i >= 0; i--) {
      tot += counter[i];
      if (tot >= i) {
        return i;
      }
    }
    return 0;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 0, 6, 1, 5}, 3},
      {{1, 3, 1}, 1},
      {{0, 1, 3, 5, 6}, 3},
  };

  for (auto& [citations, ans] : tests) {
    assert(Solution().hIndex(citations) == ans);
  }
}