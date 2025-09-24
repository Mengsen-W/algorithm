#include <cassert>
#include <cstdlib>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
  const double EPS = 1e-9;

  bool dfs(vector<double>& cards) {
    int n = cards.size();
    if (n == 1) {
      return abs(cards[0] - 24) < EPS;
    }

    // 选两张牌 x=cards[i] 和 y=cards[j]
    for (int i = 0; i < n; i++) {
      double x = cards[i];
      for (int j = i + 1; j < n; j++) {
        double y = cards[j];

        // 六种情况：加减乘除，其中减和除都有两种不同的顺序
        vector<double> candidates = {x + y, x - y, y - x, x * y};
        if (abs(y) > EPS) {  // 保证分母不为 0
          candidates.push_back(x / y);
        }
        if (abs(x) > EPS) {  // 保证分母不为 0
          candidates.push_back(y / x);
        }

        auto new_cards = cards;
        new_cards.erase(new_cards.begin() + j);  // 删除 j
        for (double res : candidates) {
          new_cards[i] = res;  // 覆盖 i
          if (dfs(new_cards)) {
            return true;
          }
        }
      }
    }
    return false;
  }

 public:
  bool judgePoint24(vector<int>& cards) {
    vector<double> a(cards.begin(), cards.end());
    return dfs(a);
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{4, 1, 8, 7}, true},
      {{1, 2, 1, 2}, false},
  };

  for (auto& [cards, expect] : tests) {
    assert(Solution().judgePoint24(cards) == expect);
  }
}