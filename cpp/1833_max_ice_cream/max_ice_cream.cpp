#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int maxIceCream(vector<int>& costs, int coins) {
    vector<int> freq(100001);
    for (int& cost : costs) {
      freq[cost]++;
    }
    int count = 0;
    for (int i = 1; i <= 100000; i++) {
      if (coins >= i) {
        int curCount = min(freq[i], coins / i);
        count += curCount;
        coins -= i * curCount;
      } else {
        break;
      }
    }
    return count;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 3, 2, 4, 1}, 7, 4},
      {{10, 6, 8, 7, 7, 8}, 5, 0},
      {{1, 6, 3, 1, 2, 5}, 20, 6},
  };

  for (auto [costs, coins, expected] : tests) {
    assert(Solution().maxIceCream(costs, coins) == expected);
  }
}
