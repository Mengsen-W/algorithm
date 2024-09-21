#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  using ll = long long;
  int edgeScore(vector<int>& edges) {
    int n = edges.size();
    vector<ll> points(n);
    for (int i = 0; i < n; i++) {
      points[edges[i]] += i;
    }
    ll max_points = -1;
    int res = -1;
    for (int i = 0; i < n; i++) {
      if (points[i] > max_points) {
        max_points = points[i];
        res = i;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 0, 0, 0, 0, 7, 7, 5}, 7},
      {{2, 0, 0, 2}, 0},
  };

  for (auto& [edges, ans] : tests) {
    assert(Solution().edgeScore(edges) == ans);
  }
}