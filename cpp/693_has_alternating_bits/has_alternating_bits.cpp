#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool hasAlternatingBits(int n) {
    long a = n ^ (n >> 1);
    return (a & (a + 1)) == 0;
  }
};

int main() {
  vector<tuple<int, bool>> tests{
      {5, true},
      {7, false},
      {11, false},
  };

  for (auto& [n, ans] : tests) {
    assert(Solution().hasAlternatingBits(n) == ans);
  }
}
