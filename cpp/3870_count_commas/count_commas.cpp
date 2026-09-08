#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    int countCommas(int n) {
        return max(n - 999, 0);
    }
};

int main() {
  vector<tuple<int, int>> tests{
      {1002, 3},
      {998, 0},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().countCommas(n) == ans);
  }
}