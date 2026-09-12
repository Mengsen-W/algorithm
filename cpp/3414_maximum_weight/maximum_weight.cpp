#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> maximumWeight(vector<vector<int>> &intervals) {
    int n = intervals.size();
    vector<tuple<int, int, int, int>> arr;
    for (int i = 0; i < n; i++) {
      int l = intervals[i][0], r = intervals[i][1], weight = intervals[i][2];
      arr.emplace_back(l, r, weight, i);
    }
    // 按照右端点大小进行排序
    sort(arr.begin(), arr.end(), [](auto &&a, auto &&b) { return get<1>(a) < get<1>(b); });

    vector<vector<long long>> dp(n + 1, vector<long long>(5));
    vector<vector<vector<int>>> indices(n + 1, vector<vector<int>>(5));
    for (int i = 0; i < n; i++) {
      auto [l, r, weight, idx] = arr[i];
      // 二分查找找到小于 l 的区间
      int k = lower_bound(arr.begin(), arr.begin() + i, l,
                          [](const tuple<int, int, int, int> &t, int val) { return get<1>(t) < val; }) -
              arr.begin();

      for (int j = 1; j < 5; j++) {
        long long s1 = dp[i][j];
        long long s2 = dp[k][j - 1] + weight;
        if (s1 > s2) {
          dp[i + 1][j] = dp[i][j];
          indices[i + 1][j] = indices[i][j];
          continue;
        }

        vector<int> newIndex = indices[k][j - 1];
        newIndex.push_back(idx);
        sort(newIndex.begin(), newIndex.end());
        if (s1 == s2 && indices[i][j] < newIndex) {
          newIndex = indices[i][j];
        }
        dp[i + 1][j] = s2;
        indices[i + 1][j] = newIndex;
      }
    }

    return indices[n][4];
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>>> tests{
      {{{1, 3, 2}, {4, 5, 2}, {1, 5, 5}, {6, 9, 3}, {6, 7, 1}, {8, 9, 1}}, {2, 3}},
      {{{5, 8, 1}, {6, 7, 7}, {4, 7, 3}, {9, 10, 6}, {7, 8, 2}, {11, 14, 3}, {3, 5, 5}}, {1, 3, 5, 6}},
  };

  for (auto &[intervals, ans] : tests) {
    assert(Solution().maximumWeight(intervals) == ans);
  }
}