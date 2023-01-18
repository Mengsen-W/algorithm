/*
 * @Date: 2021-12-20 00:46:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-20 00:56:57
 */

#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  int findRadius(vector<int>& houses, vector<int>& heaters) {
    sort(houses.begin(), houses.end());
    sort(heaters.begin(), heaters.end());
    int ans = 0;
    size_t houses_size = houses.size();
    size_t heaters_size = heaters.size();
    for (size_t i = 0, j = 0; i < houses_size; i++) {
      int curDistance = abs(houses[i] - heaters[j]);
      while (j < heaters_size - 1 &&
             abs(houses[i] - heaters[j]) >= abs(houses[i] - heaters[j + 1])) {
        j++;
        curDistance = min(curDistance, abs(houses[i] - heaters[j]));
      }
      ans = max(ans, curDistance);
    }
    return ans;
  }
};

int main() {
  {
    vector<int> houses{1, 2, 3};
    vector<int> heaters{2};
    assert(Solution().findRadius(houses, heaters) == 1);
  }
  {
    vector<int> houses{1, 2, 3, 4};
    vector<int> heaters{1, 4};
    assert(Solution().findRadius(houses, heaters) == 1);
  }
  {
    vector<int> houses{1, 5};
    vector<int> heaters{2};
    assert(Solution().findRadius(houses, heaters) == 3);
  }

  return 0;
}