#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countCompleteDayPairs(vector<int>& hours) {
    int ans = 0;
    for (int i = 1; i < hours.size(); i++) {
      for (int j = 0; j < i; j++) {
        if ((hours[i] + hours[j]) % 24 == 0) {
          ans++;
        }
      }
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