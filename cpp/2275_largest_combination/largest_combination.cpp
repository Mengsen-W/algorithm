#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int largestCombination(vector<int>& candidates) {
    // 计算从低到高第 k 个二进制位数值为 1 的元素个数
    auto maxlen = [&](int k) -> int {
      int res = 0;
      for (int num : candidates) {
        if (num & (1 << k)) {
          ++res;
        }
      }
      return res;
    };

    int res = 0;
    for (int i = 0; i < 24; ++i) {
      // 遍历二进制位
      res = max(res, maxlen(i));
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{16, 17, 71, 62, 12, 24, 14}, 4},
      {{8, 8}, 2},
  };
  
  for (auto&[candidates, ans] : tests) {
    assert(Solution().largestCombination(candidates) == ans);
  }
}