#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int minCost(string colors, vector<int>& neededTime) {
    int i = 0, len = colors.length();
    int ret = 0;
    while (i < len) {
      char ch = colors[i];
      int maxValue = 0;
      int sum = 0;

      while (i < len && colors[i] == ch) {
        maxValue = max(maxValue, neededTime[i]);
        sum += neededTime[i];
        i++;
      }
      ret += sum - maxValue;
    }
    return ret;
  }
};

int main() {
  vector<tuple<string, vector<int>, int>> tests{
      {"abaac", {1, 2, 3, 4, 5}, 3},
      {"abc", {1, 2, 3}, 0},
      {"aabaa", {1, 2, 3, 4, 1}, 2},
  };

  for (auto& [colors, neededTime, ans] : tests) {
    assert(Solution().minCost(colors, neededTime) == ans);
  }
}
