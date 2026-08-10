#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool winnerSquareGame(int n) {
    vector<int> f(n + 1);
    for (int i = 1; i <= n; ++i) {
      for (int k = 1; k * k <= i; ++k) {
        if (!f[i - k * k]) {
          f[i] = true;
          break;
        }
      }
    }

    return f[n];
  }
};

int main() {
  vector<tuple<int, bool>> tests{
      {1, true},
      {2, false},
      {7, false},
      {17, false},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().winnerSquareGame(n) == ans);
  }
}