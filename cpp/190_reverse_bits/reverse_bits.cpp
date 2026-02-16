#include <cassert>
#include <cstdint>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int reverseBits(int n) {
    uint32_t result = 0;
    for (int i = 0; i < 32; ++i) result = (result << 1) + (n >> i & 1);
    return result;
  }
};

int main() {
  vector<tuple<int, int>> tests{
      {43261596, 964176192},
      {2147483644, 1073741822},
  };

  for (auto& [n, ans] : tests) {
    assert(Solution().reverseBits(n) == ans);
  }
}
