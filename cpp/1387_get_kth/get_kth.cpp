#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  unordered_map<int, int> f;

  int getF(int x) {
    if (f.find(x) != f.end()) return f[x];
    if (x == 1) return f[x] = 0;
    if (x & 1)
      return f[x] = getF(x * 3 + 1) + 1;
    else
      return f[x] = getF(x / 2) + 1;
  }

  int getKth(int lo, int hi, int k) {
    vector<int> v;
    for (int i = lo; i <= hi; ++i) v.push_back(i);
    sort(v.begin(), v.end(), [&](int u, int v) {
      if (getF(u) != getF(v))
        return getF(u) < getF(v);
      else
        return u < v;
    });
    return v[k - 1];
  }
};

int main() {
  vector<tuple<int, int, int, int>> tests{
      {12, 15, 2, 13},
      {7, 11, 4, 7},
  };

  for (auto &[lo, hi, k, ans] : tests) {
    assert(Solution().getKth(lo, hi, k) == ans);
  }
}