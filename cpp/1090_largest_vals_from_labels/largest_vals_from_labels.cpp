/*
 * @Date: 2023-05-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-23
 * @FilePath: /algorithm/cpp/1090_largest_vals_from_labels/largest_vals_from_labels.cpp
 */

#include <cassert>
#include <numeric>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int largestValsFromLabels(vector<int>& values, vector<int>& labels, int numWanted, int useLimit) {
    int n = values.size();
    vector<int> id(n);
    iota(id.begin(), id.end(), 0);
    sort(id.begin(), id.end(), [&](int i, int j) { return values[i] > values[j]; });

    int ans = 0, choose = 0;
    unordered_map<int, int> cnt;
    for (int i = 0; i < n && choose < numWanted; ++i) {
      int label = labels[id[i]];
      if (cnt[label] == useLimit) {
        continue;
      }
      ++choose;
      ans += values[id[i]];
      ++cnt[label];
    }
    return ans;
  }
};

int main() {
  {
    vector<int> values{5, 4, 3, 2, 1};
    vector<int> labels{1, 1, 2, 2, 3};
    int numWanted = 3;
    int useLimit = 1;
    int ans = 9;
    assert(Solution().largestValsFromLabels(values, labels, numWanted, useLimit) == ans);
  }

  {
    vector<int> values{5, 4, 3, 2, 1};
    vector<int> labels{1, 3, 3, 3, 2};
    int numWanted = 3;
    int useLimit = 2;
    int ans = 12;
    assert(Solution().largestValsFromLabels(values, labels, numWanted, useLimit) == ans);
  }

  {
    vector<int> values{9, 8, 8, 7, 6};
    vector<int> labels{0, 0, 0, 1, 1};
    int numWanted = 3;
    int useLimit = 1;
    int ans = 16;
    assert(Solution().largestValsFromLabels(values, labels, numWanted, useLimit) == ans);
  }
}