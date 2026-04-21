#include <bits/fs_fwd.h>
#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
 private:
  vector<int> fa;
  vector<int> rank;
  // 路径压缩
  int find(int x) {
    if (fa[x] != x) {
      fa[x] = find(fa[x]);
    }
    return fa[x];
  }

  void Union(int x, int y) {
    x = find(x);
    y = find(y);
    if (x == y) return;
    // 按秩合并
    if (rank[x] < rank[y]) {
      swap(x, y);
    }
    fa[y] = x;
    if (rank[x] == rank[y]) {
      rank[x]++;
    }
  }

 public:
  int minimumHammingDistance(vector<int>& source, vector<int>& target, vector<vector<int>>& allowedSwaps) {
    int n = source.size();
    fa.resize(n);
    rank.resize(n, 0);
    for (int i = 0; i < n; i++) {
      fa[i] = i;
    }
    for (auto& pair : allowedSwaps) {
      Union(pair[0], pair[1]);
    }
    unordered_map<int, unordered_map<int, int>> sets;
    for (int i = 0; i < n; i++) {
      int f = find(i);
      sets[f][source[i]]++;
    }
    int ans = 0;
    for (int i = 0; i < n; i++) {
      int f = find(i);
      if (sets[f][target[i]] > 0) {
        sets[f][target[i]]--;
      } else {
        ans++;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, vector<vector<int>>, int>> tests{
      {{1, 2, 3, 4}, {2, 1, 4, 5}, {{0, 1}, {2, 3}}, 1},
      {{1, 2, 3, 4}, {1, 3, 2, 4}, {}, 2},
      {{5, 1, 2, 4, 3}, {1, 5, 4, 2, 3}, {{0, 4}, {4, 2}, {1, 3}, {1, 4}}, 0},
  };

  for (auto& [source, target, allowedSwaps, ans] : tests) {
    assert(Solution().minimumHammingDistance(source, target, allowedSwaps) == ans);
  }
  return 0;
}
