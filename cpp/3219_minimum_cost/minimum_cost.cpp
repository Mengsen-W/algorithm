#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long minimumCost(int m, int n, vector<int>& horizontalCut, vector<int>& verticalCut) {
    sort(horizontalCut.begin(), horizontalCut.end());
    sort(verticalCut.begin(), verticalCut.end());
    long long h = 1, v = 1;
    long long res = 0;
    while (!horizontalCut.empty() || !verticalCut.empty()) {
      if (verticalCut.empty() || (!horizontalCut.empty() && horizontalCut.back() > verticalCut.back())) {
        res += horizontalCut.back() * h;
        horizontalCut.pop_back();
        v++;
      } else {
        res += verticalCut.back() * v;
        verticalCut.pop_back();
        h++;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int, vector<int>, vector<int>, long long>> tests{
      {3, 2, {1, 3}, {5}, 13},
      {2, 2, {7}, {4}, 15},
  };

  for (auto& [m, n, horizontalCut, verticalCut, ans] : tests) {
    assert(Solution().minimumCost(m, n, horizontalCut, verticalCut) == ans);
  }
}