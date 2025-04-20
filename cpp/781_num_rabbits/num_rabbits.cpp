#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int numRabbits(vector<int>& answers) {
    unordered_map<int, int> count;
    for (int y : answers) {
      ++count[y];
    }
    int ans = 0;
    for (auto& [y, x] : count) {
      ans += (x + y) / (y + 1) * (y + 1);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 1, 2}, 5},
      {{10, 10, 10}, 11},
      {{}, 0},
  };

  for (auto& [answers, ans] : tests) {
    assert(Solution().numRabbits(answers) == ans);
  }
  return 0;
}