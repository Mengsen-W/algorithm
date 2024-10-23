#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countCompleteDayPairs(vector<int>& hours) {
    long long ans = 0;
    vector<int> cnt(24);
    for (int hour : hours) {
      ans += cnt[(24 - hour % 24) % 24];
      cnt[hour % 24]++;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{12, 12, 30, 24, 24}, 2},
      {{72, 48, 24, 3}, 3},
  };

  for (auto &[hours, ans] : tests) {
    assert(Solution().countCompleteDayPairs(hours) == ans);
  }
}