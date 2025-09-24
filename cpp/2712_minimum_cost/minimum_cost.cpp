#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  using ll = long long;
  long long minimumCost(string s) {
    int n = s.size();
    ll res = 0;
    for (int i = 1; i < n; i++) {
      if (s[i] != s[i - 1]) {
        res += min(i, n - i);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, long long>> tests{
      {"0011", 2},
      {"010101", 9},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().minimumCost(s) == ans);
  }
}