/*
 * @Date: 2023-09-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-27
 * @FilePath: /algorithm/cpp/1333_filter_restaurants/filter_restaurants.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> filterRestaurants(vector<vector<int>> &restaurants, int veganFriendly, int maxPrice, int maxDistance) {
    int n = restaurants.size();
    vector<vector<int>> filtered;
    for (int i = 0; i < n; i++) {
      if (restaurants[i][3] <= maxPrice && restaurants[i][4] <= maxDistance && !(veganFriendly && !restaurants[i][2])) {
        filtered.push_back(restaurants[i]);
      }
    }
    sort(filtered.begin(), filtered.end(),
         [](vector<int> &v1, vector<int> &v2) -> bool { return v1[1] > v2[1] || (v1[1] == v2[1] && v1[0] > v2[0]); });
    vector<int> res;
    for (auto &v : filtered) {
      res.push_back(v[0]);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int, int, vector<int>>> tests{
      {
          {{1, 4, 1, 40, 10}, {2, 8, 0, 50, 5}, {3, 8, 1, 30, 4}, {4, 10, 0, 10, 3}, {5, 1, 1, 15, 1}},
          1,
          50,
          10,
          {3, 1, 5},
      },
      {
          {{1, 4, 1, 40, 10}, {2, 8, 0, 50, 5}, {3, 8, 1, 30, 4}, {4, 10, 0, 10, 3}, {5, 1, 1, 15, 1}},
          0,
          50,
          10,
          {4, 3, 2, 1, 5},
      },
      {
          {{1, 4, 1, 40, 10}, {2, 8, 0, 50, 5}, {3, 8, 1, 30, 4}, {4, 10, 0, 10, 3}, {5, 1, 1, 15, 1}},
          0,
          30,
          3,
          {4, 5},
      },
  };

  for (auto &[restaurants, veganFriendly, maxPrice, maxDistance, expected] : tests) {
    assert(Solution().filterRestaurants(restaurants, veganFriendly, maxPrice, maxDistance) == expected);
  }
}