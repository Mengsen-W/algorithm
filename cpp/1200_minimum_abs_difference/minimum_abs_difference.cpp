#include <algorithm>
#include <cassert>
#include <climits>
#include <tuple>
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
  vector<tuple<vector<int>, vector<vector<int>>>> tests{
      {{4, 2, 1, 3}, {{1, 2}, {2, 3}, {3, 4}}},
      {{1, 3, 6, 10, 15}, {{1, 3}}},
      {{3, 8, -10, 23, 19, -4, -14, 27}, {{-14, -10}, {19, 23}, {23, 27}}},
  };

  for (auto [arr, expected] : tests) {
    assert(Solution().minimumAbsDifference(arr) == expected);
  }
}