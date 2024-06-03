#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> distributeCandies(int candies, int numPeople) {
    int n = numPeople;
    // how many people received complete gifts
    int p = (int)(sqrt(2 * candies + 0.25) - 0.5);
    int remaining = (int)(candies - (p + 1) * p * 0.5);
    int rows = p / n, cols = p % n;

    vector<int> d(n, 0);
    for (int i = 0; i < n; ++i) {
      // complete rows
      d[i] = (i + 1) * rows + (int)(rows * (rows - 1) * 0.5) * n;
      // cols in the last row
      if (i < cols) {
        d[i] += i + 1 + rows * n;
      }
    }
    // remaining candies
    d[cols] += remaining;
    return d;
  }
};

int main() {
  vector<tuple<int, int, vector<int>>> tests{
      {7, 4, {1, 2, 3, 1}},
      {10, 3, {5, 2, 3}},
  };

  for (auto &[candies, numPeople, ans] : tests) {
    assert(Solution().distributeCandies(candies, numPeople) == ans);
  }
}
