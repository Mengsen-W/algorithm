#include <cassert>
#include <tuple>
#include <vector>

class Solution {
public:
    long long countCommas(long long n) {
        long long p = 1000, res = 0;
        while (p <= n) {
            res += n - p + 1;
            p *= 1000;
        }
        return res;
    }
};

int main() {
  std::vector<std::tuple<long long, long long>> tests{
      {1002, 3},
      {998, 0},
  };

  for (auto [n, expected] : tests) {
    assert(Solution().countCommas(n) == expected);
  }
}