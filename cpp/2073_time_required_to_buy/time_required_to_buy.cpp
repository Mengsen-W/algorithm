
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int timeRequiredToBuy(vector<int>& tickets, int k) {
    int n = tickets.size();
    int res = 0;
    for (int i = 0; i < n; ++i) {
      // 遍历计算每个人所需时间
      if (i <= k) {
        res += min(tickets[i], tickets[k]);
      } else {
        res += min(tickets[i], tickets[k] - 1);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{2, 3, 2}, 2, 6},
      {{5, 1, 1, 1}, 0, 8},
  };

  for (auto &[tickets, k, ans] : tests) {
    assert(Solution().timeRequiredToBuy(tickets, k) == ans);
  }
}
