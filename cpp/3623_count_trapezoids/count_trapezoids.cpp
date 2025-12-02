#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int countTrapezoids(vector<vector<int>>& points) {
    unordered_map<int, int> pointNum;
    const int mod = 1e9 + 7;
    long long ans = 0, sum = 0;
    for (auto& point : points) {
      pointNum[point[1]]++;
    }
    for (auto& [_, pNum] : pointNum) {
      long long edge = (long long)pNum * (pNum - 1) / 2;
      ans += edge * sum;
      sum += edge;
    }
    return ans % mod;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 0}, {2, 0}, {3, 0}, {2, 2}, {3, 2}}, 3},
      {{{0, 0}, {1, 0}, {0, 1}, {2, 1}}, 1},
  };

  for (auto& [points, expect] : tests) {
    assert(Solution().countTrapezoids(points) == expect);
  }
}