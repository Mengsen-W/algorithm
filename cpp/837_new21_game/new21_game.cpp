#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  double new21Game(int n, int k, int maxPts) {
    vector<double> f(n + 1);
    double s = 0;
    for (int i = n; i >= 0; i--) {
      f[i] = i >= k ? 1 : s / maxPts;
      // 当前循环计算的是 f[i+1] + ... + f[i+maxPts]
      // 下个循环计算的是 f[i] + ... + f[i+maxPts-1]，多了 f[i]，少了 f[i+maxPts]
      s += f[i];
      if (i + maxPts <= n) {
        s -= f[i + maxPts];
      }
    }
    return f[0];
  }
};

int main() {
  vector<tuple<int, int, int, double>> tests{
      {10, 1, 10, 1.0000},
      {6, 1, 10, 0.60000},
      {21, 17, 10, 0.73278},
  };

  for (auto &[n, k, maxPts, ans] : tests) {
    assert(Solution().new21Game(n, k, maxPts) == ans);
  }
}