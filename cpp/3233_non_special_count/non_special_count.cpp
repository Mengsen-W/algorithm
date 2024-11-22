#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int nonSpecialCount(int l, int r) {
    int n = sqrt(r);
    vector<int> v(n + 1);
    int res = r - l + 1;
    for (int i = 2; i <= n; i++) {
      if (v[i] == 0) {
        if (i * i >= l && i * i <= r) {
          res--;
        }
        for (int j = i * 2; j <= n; j += i) {
          v[j] = 1;
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int, int>> tests{
      {5, 7, 3},
      {4, 16, 11},
  };

  for (auto &[l, r, ans] : tests) {
    assert(Solution().nonSpecialCount(l, r) == ans);
  }
}
