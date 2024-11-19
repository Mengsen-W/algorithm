#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> shortestDistanceAfterQueries(int n, vector<vector<int>>& queries) {
    vector<int> roads(n);
    iota(roads.begin(), roads.end(), 1);
    vector<int> res;
    int dist = n - 1;
    for (auto& query : queries) {
      int k = roads[query[0]];
      roads[query[0]] = query[1];
      while (k != -1 && k < query[1]) {
        int t = roads[k];
        roads[k] = -1;
        k = t;
        dist--;
      }
      res.push_back(dist);
    }
    return res;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>>> tests{
      {5, {{2, 4}, {0, 2}, {0, 4}}, {3, 2, 1}},
      {4, {{0, 3}, {0, 2}}, {1, 1}},
  };

  for (auto& [n, queries, ans] : tests) {
    assert(Solution().shortestDistanceAfterQueries(n, queries) == ans);
  }
}