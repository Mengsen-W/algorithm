/*
 * @Date: 2021-11-01 01:10:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-01 01:13:10
 */

#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int distributeCandies(vector<int> candyType) {
    return min(unordered_set<int>(candyType.begin(), candyType.end()).size(), candyType.size() / 2);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 1, 2, 2, 3, 3}, 3},
      {{1, 1, 2, 3}, 2},
  };

  for (auto &[candyType, ans] : tests) {
    assert(Solution().distributeCandies(candyType) == ans);
  }
}