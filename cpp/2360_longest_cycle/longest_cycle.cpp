#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestCycle(vector<int>& edges) {
    int n = edges.size();
    vector<int> label(n);
    int current_label = 0, ans = -1;
    for (int i = 0; i < n; ++i) {
      if (label[i]) {
        continue;
      }
      int pos = i, start_label = current_label;
      while (pos != -1) {
        ++current_label;
        // 如果遇到了已经遍历过的节点
        if (label[pos]) {
          // 如果该节点是这一次 i 循环中遍历的，说明找到了新的环，更新答案
          if (label[pos] > start_label) {
            ans = max(ans, current_label - label[pos]);
          }
          break;
        }
        label[pos] = current_label;
        pos = edges[pos];
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 3, 4, 2, 3}, 3},
      {{2, -1, 3, 1}, -1},
  };

  for (auto &[edges, ans] : tests) {
    assert(Solution().longestCycle(edges) == ans);
  }
}