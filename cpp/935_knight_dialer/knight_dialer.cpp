#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
  static constexpr int mod = 1e9 + 7;

 public:
  int knightDialer(int n) {
    vector<vector<int>> moves = {{4, 6}, {6, 8}, {7, 9}, {4, 8}, {3, 9, 0}, {}, {1, 7, 0}, {2, 6}, {1, 3}, {2, 4}};
    vector<vector<int>> d(2, vector<int>(10, 0));
    fill(d[1].begin(), d[1].end(), 1);
    for (int i = 2; i <= n; i++) {
      int x = i & 1;
      for (int j = 0; j < 10; j++) {
        d[x][j] = 0;
        for (int k : moves[j]) {
          d[x][j] = (d[x][j] + d[x ^ 1][k]) % mod;
        }
      }
    }
    int res = 0;
    for (auto x : d[n % 2]) {
      res = (res + x) % mod;
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int>> tests{
      {1, 10},
      {2, 20},
      {3131, 136006598},
  };

  for (auto &[n ,ans] : tests) {
    assert(Solution().knightDialer(n) == ans);
  }
}