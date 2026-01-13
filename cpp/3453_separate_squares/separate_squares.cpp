#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  double separateSquares(vector<vector<int>>& squares) {
    long long total_area = 0;
    vector<tuple<int, int, int>> events;
    for (const auto& sq : squares) {
      int y = sq[1], l = sq[2];
      total_area += (long long)l * l;
      events.emplace_back(y, l, 1);
      events.emplace_back(y + l, l, -1);
    }
    // 按照 y 坐标进行排序
    sort(events.begin(), events.end(), [](const auto& a, const auto& b) { return get<0>(a) < get<0>(b); });

    double covered_width = 0;  // 当前扫描线下所有底边之和
    double curr_area = 0;      // 当前累计面积
    double prev_height = 0;    // 前一个扫描线的高度
    for (const auto& [y, l, delta] : events) {
      int diff = y - prev_height;
      // 两条扫面线之间新增的面积
      double area = covered_width * diff;
      // 如果加上这部分面积超过总面积的一半
      if (2LL * (curr_area + area) >= total_area) {
        return prev_height + (total_area - 2.0 * curr_area) / (2.0 * covered_width);
      }
      // 更新宽度：开始事件加宽度，结束事件减宽度
      covered_width += delta * l;
      curr_area += area;
      prev_height = y;
    }

    return 0.0;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, double>> tests{
      {{{0, 0, 1}, {2, 2, 1}}, 1.0},
      {{{0, 0, 2}, {1, 1, 1}}, 1.16667},
  };

  for (auto [squares, expected] : tests) {
    assert(abs(Solution().separateSquares(squares) - expected) < 1e-5);
  }

  return 0;
}