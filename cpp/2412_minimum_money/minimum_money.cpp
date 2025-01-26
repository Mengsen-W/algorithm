#include <cassert>
#include <cstddef>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long minimumMoney(vector<vector<int>>& transactions) {
    long long total_lose = 0;
    int res = 0;
    for (auto& t : transactions) {
      auto [cost, cashback] = pair(t[0], t[1]);
      total_lose += max(cost - cashback, 0);
      res = max(res, min(cost, cashback));
    }
    return total_lose + res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, long long>> tests{
      {{{2, 1}, {5, 0}, {4, 2}}, 10},
      {{{3, 0}, {0, 3}}, 3},
  };

  for (auto& [transactions, ans] : tests) {
    assert(Solution().minimumMoney(transactions) == ans);
  }
}