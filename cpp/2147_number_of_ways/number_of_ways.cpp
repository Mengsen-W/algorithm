#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 private:
  static constexpr int mod = 1000000007;

 public:
  int numberOfWays(string corridor) {
    int n = corridor.size();
    int prev = -1, cnt = 0, ans = 1;
    for (int i = 0; i < n; ++i) {
      if (corridor[i] == 'S') {
        ++cnt;
        if (cnt >= 3 && cnt % 2 == 1) {
          ans = static_cast<long long>(ans) * (i - prev) % mod;
        }
        prev = i;
      }
    }
    if (cnt < 2 || cnt & 1) {
      ans = 0;
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"SSPPSPS", 3},
      {"PPSPSP", 1},
      {"S", 0},
  };

  for (auto &[corridor, ans] : tests) {
    assert(Solution().numberOfWays(corridor) == ans);
  }
}