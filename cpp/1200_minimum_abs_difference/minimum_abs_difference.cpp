/*
 * @Date: 2022-07-04
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-04
 * @FilePath: /algorithm/1200_minimum_abs_difference/minimum_abs_difference.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> minimumAbsDifference(vector<int>& arr) {
    int n = arr.size();
    sort(arr.begin(), arr.end());

    int best = INT_MAX;
    vector<vector<int>> ans;
    for (int i = 0; i < n - 1; ++i) {
      if (int delta = arr[i + 1] - arr[i]; delta < best) {
        best = delta;
        ans = {{arr[i], arr[i + 1]}};
      } else if (delta == best) {
        ans.emplace_back(initializer_list<int>{arr[i], arr[i + 1]});
      }
    }

    return ans;
  }
};

int main() {
  {
    vector<int> arr{4, 2, 1, 3};
    vector<vector<int>> ans{{1, 2}, {2, 3}, {3, 4}};
    assert(Solution().minimumAbsDifference(arr) == ans);
  }

  {
    vector<int> arr{1, 3, 6, 10, 15};
    vector<vector<int>> ans{{1, 3}};
    assert(Solution().minimumAbsDifference(arr) == ans);
  }

  {
    vector<int> arr{3, 8, -10, 23, 19, -4, -14, 27};
    vector<vector<int>> ans{{-14, -10}, {19, 23}, {23, 27}};
    assert(Solution().minimumAbsDifference(arr) == ans);
  }
}