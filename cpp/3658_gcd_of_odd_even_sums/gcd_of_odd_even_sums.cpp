#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int gcdOfOddEvenSums(int n) { return n; }
};

int main() {
  vector<tuple<int, int>> tests{
      {4, 4},
      {5, 5},
  };

  for (auto [n, expected] : tests) {
    assert(Solution().gcdOfOddEvenSums(n) == expected);
  }
}