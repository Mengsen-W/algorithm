#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int findTheDistanceValue(vector<int>& arr1, vector<int>& arr2, int d) {
    sort(arr2.begin(), arr2.end());
    int cnt = 0;
    for (auto& x : arr1) {
      unsigned p = lower_bound(arr2.begin(), arr2.end(), x) - arr2.begin();
      bool ok = true;
      if (p < arr2.size()) {
        ok &= (arr2[p] - x > d);
      }
      if (p - 1 >= 0 && p - 1 <= arr2.size()) {
        ok &= (x - arr2[p - 1] > d);
      }
      cnt += ok;
    }
    return cnt;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int, int>> testCases = {
      {{4, 5, 8}, {10, 9, 1, 8}, 2, 2},
      {{1, 4, 2, 3}, {-4, -3, 6, 10, 20, 30}, 3, 2},
      {{2, 1, 100, 3}, {-5, -2, 10, -3, 7}, 6, 1},
  };
  for (auto& [arr1, arr2, d, ans] : testCases) {
    assert(Solution().findTheDistanceValue(arr1, arr2, d) == ans);
  }
  return 0;
}