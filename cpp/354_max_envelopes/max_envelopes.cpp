/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-04 09:44:24
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-04 09:55:09
 */

#include <algorithm>
#include <cassert>
#include <set>
#include <vector>

using namespace std;

int max_envelopes(vector<vector<int>> &&envelopes) {
  sort(envelopes.begin(), envelopes.end(),
       [](const vector<int> &a, const vector<int> &b) -> bool {
         if (a[0] == b[0]) return a[1] > b[1];
         return a[0] < b[0];
       });
  set<int> a;
  for (auto e : envelopes) {
    int v = e[1];
    auto it = lower_bound(a.begin(), a.end(), v);
    if (it != a.end()) {
      a.erase(it);
    }
    a.insert(v);
  }
  return a.size();
}

int main() {
  assert(3 ==
         max_envelopes(vector<vector<int>>{{5, 4}, {6, 4}, {6, 7}, {2, 3}}));
  return 0;
}