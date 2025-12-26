#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int bestClosingTime(string customers) {
    int n = customers.size();
    int suf = 0;
    int pre = 0;
    int min_cost = 0, res = 0;
    for (int i = 0; i <= n; i++) {
      if (min_cost > suf + pre) {
        min_cost = suf + pre;
        res = i;
      }
      if (i < n && customers[i] == 'N') {
        pre++;
      } else {
        suf--;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"YYNY", 2},
      {"NNNNN", 0},
      {"YYYY", 4},
  };

  for (auto& [customers, ans] : tests) {
    assert(Solution().bestClosingTime(customers) == ans);
  }
}