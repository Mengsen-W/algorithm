/*
 * @Date: 2022-09-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-03
 * @FilePath: /algorithm/646_find_longest_chain/find_longest_chain.cpp
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int findLongestChain(vector<vector<int>> &pairs) {
    int curr = INT_MIN, res = 0;
    sort(pairs.begin(), pairs.end(), [](const vector<int> &a, const vector<int> &b) { return a[1] < b[1]; });
    for (auto &p : pairs) {
      if (curr < p[0]) {
        curr = p[1];
        res++;
      }
    }
    return res;
  }
};

int main() {
  vector<vector<int>> pairs{{1, 2}, {2, 3}, {3, 4}};
  int ans = 2;
  assert(Solution().findLongestChain(pairs) == ans);
}