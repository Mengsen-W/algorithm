#include <cassert>
#include <queue>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  bool canReach(vector<int>& arr, int start) {
    if (arr[start] == 0) {
      return true;
    }

    int n = arr.size();
    vector<bool> used(n);
    queue<int> q;
    q.push(start);
    used[start] = true;

    while (!q.empty()) {
      int u = q.front();
      q.pop();
      if (u + arr[u] < n && !used[u + arr[u]]) {
        if (arr[u + arr[u]] == 0) {
          return true;
        }
        q.push(u + arr[u]);
        used[u + arr[u]] = true;
      }
      if (u - arr[u] >= 0 && !used[u - arr[u]]) {
        if (arr[u - arr[u]] == 0) {
          return true;
        }
        q.push(u - arr[u]);
        used[u - arr[u]] = true;
      }
    }
    return false;
  }
};

int main() {
  vector<tuple<vector<int>, int, bool>> tests{
      {{4, 2, 3, 0, 3, 1, 2}, 5, true},
      {{4, 2, 3, 0, 3, 1, 2}, 0, true},
      {{3, 0, 2, 1, 2}, 2, false},
  };

  for (auto& [arr, start, expect] : tests) {
    assert(Solution().canReach(arr, start) == expect);
  }
}
