#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int numRescueBoats(vector<int> &people, int limit) {
    int ans = 0;
    sort(people.begin(), people.end());
    int light = 0, heavy = people.size() - 1;
    while (light <= heavy) {
      if (people[light] + people[heavy] > limit) {
        --heavy;
      } else {
        ++light;
        --heavy;
      }
      ++ans;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2}, 3, 1},
      {{3, 2, 2, 1}, 3, 3},
      {{3, 5, 3, 4}, 5, 4},
  };

  for (auto &[perple, limit, ans] : tests) {
    assert(Solution().numRescueBoats(perple, limit) == ans);
  }
}