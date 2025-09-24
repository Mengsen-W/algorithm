
#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int countBalls(int lowLimit, int highLimit) {
    unordered_map<int, int> count;
    int res = 0;
    for (int i = lowLimit; i <= highLimit; i++) {
      int box = 0, x = i;
      while (x) {
        box += x % 10;
        x /= 10;
      }
      count[box]++;
      res = max(res, count[box]);
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int, int>> tests{
      {1, 10, 2},
      {5, 15, 2},
      {19, 28, 2},
  };

  for (auto& [lowLimit, highLimit, ans] : tests) {
    assert(Solution().countBalls(lowLimit, highLimit) == ans);
  }
}