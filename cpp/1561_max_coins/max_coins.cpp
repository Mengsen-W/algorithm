#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxCoins(vector<int>& piles) {
    sort(piles.begin(), piles.end());
    int length = piles.size();
    int rounds = length / 3;
    int coins = 0;
    int index = length - 2;
    for (int i = 0; i < rounds; i++) {
      coins += piles[index];
      index -= 2;
    }
    return coins;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 4, 1, 2, 7, 8}, 9},
      {{2, 4, 5}, 4},
      {{9, 8, 7, 6, 5, 1, 2, 3, 4}, 18},
  };
  
  for (auto &[piles, ans] : tests ) {
    assert(Solution().maxCoins(piles) == ans);
  }
}
