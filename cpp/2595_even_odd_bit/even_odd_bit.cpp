#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> evenOddBit(int n) {
    vector<int> res = {0, 0};
    int i = 0;
    while (n) {
      res[i] += n & 1;
      n >>= 1;
      i ^= 1;
    }
    return res;
  }
};

int main() {
  vector<tuple<int, vector<int>>> tests{
      {17, {2, 0}},
      {2, {0, 1}},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().evenOddBit(n) == ans);
  }
}