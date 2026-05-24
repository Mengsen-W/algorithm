#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 private:
  vector<int> f;

 public:
  void dfs(vector<int>& arr, int id, int d, int n) {
    if (f[id] != -1) {
      return;
    }
    f[id] = 1;
    for (int i = id - 1; i >= 0 && id - i <= d && arr[id] > arr[i]; --i) {
      dfs(arr, i, d, n);
      f[id] = max(f[id], f[i] + 1);
    }
    for (int i = id + 1; i < n && i - id <= d && arr[id] > arr[i]; ++i) {
      dfs(arr, i, d, n);
      f[id] = max(f[id], f[i] + 1);
    }
  }

  int maxJumps(vector<int>& arr, int d) {
    int n = arr.size();
    f.resize(n, -1);
    for (int i = 0; i < n; ++i) {
      dfs(arr, i, d, n);
    }
    return *max_element(f.begin(), f.end());
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12}, 2, 4},
      {{3, 3, 3, 3, 3}, 3, 1},
      {{7, 6, 5, 4, 3, 2, 1}, 1, 7},
      {{7, 1, 7, 1, 7, 1}, 2, 2},
      {{66}, 1, 1},
  };

  for (auto& [arr, d, ans] : tests) {
    assert(Solution().maxJumps(arr, d) == ans);
  }
  return 0;
}