/*
 * @Date: 2023-09-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-28
 * @FilePath: /algorithm/cpp/2251_full_bloom_flowers/full_bloom_flowers.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> fullBloomFlowers(vector<vector<int>> &flowers, vector<int> &persons) {
    int n = flowers.size();
    vector<int> starts(n), ends(n);
    for (int i = 0; i < n; ++i) {
      starts[i] = flowers[i][0];
      ends[i] = flowers[i][1];
    }
    sort(starts.begin(), starts.end());
    sort(ends.begin(), ends.end());
    int m = persons.size();
    vector<int> ans(m);
    for (int i = 0; i < m; ++i) {
      int x = upper_bound(starts.begin(), starts.end(), persons[i]) - starts.begin();
      int y = lower_bound(ends.begin(), ends.end(), persons[i]) - ends.begin();
      ans[i] = x - y;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>, vector<int>>> tests{
      {{{1, 6}, {3, 7}, {9, 12}, {4, 13}}, {2, 3, 7, 11}, {1, 2, 2, 2}},
      {{{1, 10}, {3, 3}}, {3, 3, 2}, {2, 2, 1}},
  };

  for (auto &[flowers, persons, ans] : tests) {
    assert(Solution().fullBloomFlowers(flowers, persons) == ans);
  }
}