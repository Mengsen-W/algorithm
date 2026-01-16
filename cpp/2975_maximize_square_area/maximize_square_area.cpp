#include <algorithm>
#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
  unordered_set<int> getEdges(vector<int>& fences, int border) {
    unordered_set<int> st;
    fences.push_back(1);
    fences.push_back(border);
    sort(fences.begin(), fences.end());
    for (int i = 0; i < fences.size(); i++) {
      for (int j = i + 1; j < fences.size(); j++) {
        st.insert(fences[j] - fences[i]);
      }
    }
    return st;
  }

 public:
  int maximizeSquareArea(int m, int n, vector<int>& hFences, vector<int>& vFences) {
    auto hEdges = getEdges(hFences, m);
    auto vEdges = getEdges(vFences, n);
    int res = 0;
    for (auto e : hEdges) {
      if (vEdges.contains(e)) {
        res = max(res, e);
      }
    }
    if (res == 0) {
      res = -1;
    } else {
      res = 1ll * res * res % 1000000007;
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int, vector<int>, vector<int>, int>> tests{
      {4, 3, {2, 3}, {2}, 4},
      {6, 7, {2}, {4}, -1},
  };

  for (auto [m, n, hFences, vFences, expected] : tests) {
    assert(Solution().maximizeSquareArea(m, n, hFences, vFences) == expected);
  }
}