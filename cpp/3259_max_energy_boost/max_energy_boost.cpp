#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  using ll = long long;
  long long maxEnergyBoost(vector<int>& energyDrinkA, vector<int>& energyDrinkB) {
    int n = energyDrinkA.size();
    vector<vector<ll>> d(n + 1, vector<ll>(2, 0));
    for (int i = 1; i <= n; i++) {
      d[i][0] = d[i - 1][0] + energyDrinkA[i - 1];
      d[i][1] = d[i - 1][1] + energyDrinkB[i - 1];
      if (i >= 2) {
        d[i][0] = max(d[i][0], d[i - 2][1] + energyDrinkA[i - 1]);
        d[i][1] = max(d[i][1], d[i - 2][0] + energyDrinkB[i - 1]);
      }
    }
    return max(d[n][0], d[n][1]);
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, long long>> tests{
      {{1, 3, 1}, {3, 1, 1}, 5},
      {{4, 1, 1}, {1, 1, 3}, 7},
  };

  for (auto &[energyDrinkA, energyDrinkB, ans] : tests) {
    assert(Solution().maxEnergyBoost(energyDrinkA, energyDrinkB) == ans);
  }
}