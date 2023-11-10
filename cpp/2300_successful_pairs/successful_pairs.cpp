/*
 * @Date: 2023-11-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-10
 * @FilePath: /algorithm/cpp/2300_successful_pairs/successful_pairs.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> successfulPairs(vector<int>& spells, vector<int>& potions, long long success) {
    vector<int> res(spells.size());
    vector<int> idx(spells.size());
    iota(idx.begin(), idx.end(), 0);
    sort(idx.begin(), idx.end(), [&](int a, int b) { return spells[a] < spells[b]; });
    sort(potions.begin(), potions.end(), [](int a, int b) { return a > b; });
    for (int i = 0, j = 0; i < spells.size(); ++i) {
      int p = idx[i];
      int v = spells[p];
      while (j < potions.size() && (long long)potions[j] * v >= success) {
        ++j;
      }
      res[p] = j;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, long long, vector<int>>> tests{
      {{5, 1, 3}, {1, 2, 3, 4, 5}, 7, {4, 0, 3}},
      {{3, 1, 2}, {8, 5, 8}, 16, {2, 0, 2}},
  };

  for (auto& [spells, potions, success, ans] : tests) {
    assert(Solution().successfulPairs(spells, potions, success) == ans);
  }
}