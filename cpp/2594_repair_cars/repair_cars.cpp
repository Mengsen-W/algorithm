/*
 * @Date: 2023-09-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-07
 * @FilePath: /algorithm/cpp/2594_repair_cars/repair_cars.cpp
 */

#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  using ll = long long;
  long long repairCars(vector<int>& ranks, int cars) {
    ll l = 1, r = 1ll * ranks[0] * cars * cars;
    auto check = [&](ll m) {
      ll cnt = 0;
      for (auto x : ranks) {
        cnt += sqrt(m / x);
      }
      return cnt >= cars;
    };
    while (l < r) {
      ll m = (l + r) >> 1;
      if (check(m)) {
        r = m;
      } else {
        l = m + 1;
      }
    }
    return l;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests = {
      {{4, 2, 3, 1}, 10, 16},
      {{5, 1, 8}, 6, 16},
  };

  for (auto& [ranks, cars, expect] : tests) {
    assert(Solution().repairCars(ranks, cars) == expect);
  }
}
