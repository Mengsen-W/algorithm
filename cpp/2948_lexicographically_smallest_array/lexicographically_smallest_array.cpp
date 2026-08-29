#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> lexicographicallySmallestArray(vector<int>& nums, int limit) {
    int n = nums.size();
    vector<int> ans(n, 0);

    // 将元素值与原下标绑定
    vector<pair<int, int>> arr;
    for (int i = 0; i < n; i++) {
      arr.push_back({nums[i], i});
    }

    // 按元素值升序排序
    sort(arr.begin(), arr.end());

    vector<int> values, indices;
    for (auto& p : arr) {
      values.push_back(p.first);
      indices.push_back(p.second);
    }

    int i = 0;
    while (i < n) {
      int start = i;

      // 当前连通块中的原下标
      vector<int> groupIndices;

      // 当前连通块中的元素值
      vector<int> groupValues;

      while (i < n && (i == start || values[i] - values[i - 1] <= limit)) {
        groupIndices.push_back(indices[i]);
        groupValues.push_back(values[i]);
        i++;
      }

      // 由于元素值数组已经有序，这里不需要再排序
      sort(groupIndices.begin(), groupIndices.end());

      // 为得到字典序最小的结果，将较小元素放到较小下标处
      for (int k = 0; k < groupIndices.size(); k++) {
        ans[groupIndices[k]] = groupValues[k];
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<int>>> tests{
      {{1, 5, 3, 9, 8}, 2, {1, 3, 5, 8, 9}},
      {{1, 7, 6, 18, 2, 1}, 3, {1, 6, 7, 18, 1, 2}},
      {{1, 7, 28, 19, 10}, 3, {1, 7, 28, 19, 10}},
  };

  for (auto &[nums, limit, ans] : tests) {
    assert(Solution().lexicographicallySmallestArray(nums, limit) == ans);
  }
}