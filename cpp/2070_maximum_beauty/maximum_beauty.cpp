#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> maximumBeauty(vector<vector<int>>& items, vector<int>& queries) {
    // 将物品按价格升序排序
    sort(items.begin(), items.end(), [](auto&& item1, auto&& item2) { return item1[0] < item2[0]; });
    int n = items.size();
    // 按定义修改美丽值
    for (int i = 1; i < n; ++i) {
      items[i][1] = max(items[i][1], items[i - 1][1]);
    }
    // 二分查找处理查询
    auto query = [&](int q) -> int {
      int l = 0, r = n;
      while (l < r) {
        int mid = l + (r - l) / 2;
        if (items[mid][0] > q) {
          r = mid;
        } else {
          l = mid + 1;
        }
      }
      if (l == 0) {
        // 此时所有物品价格均大于查询价格
        return 0;
      } else {
        // 返回小于等于查询价格的物品的最大美丽值
        return items[l - 1][1];
      }
    };

    vector<int> res;
    for (int q : queries) {
      res.push_back(query(q));
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>, vector<int>>> tests{
      {{{1, 2}, {3, 2}, {2, 4}, {5, 6}, {3, 5}}, {1, 2, 3, 4, 5, 6}, {2, 4, 5, 5, 6, 6}},
      {{{1, 2}, {1, 2}, {1, 3}, {1, 4}}, {1}, {4}},
      {{{10, 1000}}, {5}, {0}},
  };

  for (auto& [items, queries, ans] : tests) {
    assert(Solution().maximumBeauty(items, queries) == ans);
  }
}