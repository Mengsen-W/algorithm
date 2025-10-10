#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumEnergy(vector<int>& energy, int k) {
    int n = energy.size(), ans = INT_MIN;
    for (int i = n - k; i < n; i++) {
      int sum = 0;
      for (int j = i; j >= 0; j -= k) {
        sum += energy[j];
        ans = max(ans, sum);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{5, 2, -10, -5, 1}, 3, 3},
      {{-2, -3, -1}, 2, -1},
  };

  for (auto& [energy, k, ans] : tests) {
    assert(Solution().maximumEnergy(energy, k) == ans);
  }
}
