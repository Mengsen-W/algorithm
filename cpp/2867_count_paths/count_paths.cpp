/*
 * @Date: 2024-02-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-27
 * @FilePath: /algorithm/cpp/2867_count_paths/count_paths.cpp
 */

#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

const int N = 100001;
bool isPrime[N];
int init = []() {
  // 埃氏筛
  fill(begin(isPrime), end(isPrime), true);
  isPrime[1] = false;
  for (int i = 2; i * i < N; i++) {
    if (isPrime[i]) {
      for (int j = i * i; j < N; j += i) {
        isPrime[j] = false;
      }
    }
  }
  return 0;
}();

class Solution {
 private:
  void dfs(const vector<vector<int>>& G, vector<int>& seen, int i, int pre) {
    seen.push_back(i);
    for (int j : G[i]) {
      if (j != pre && !isPrime[j]) {
        dfs(G, seen, j, i);
      }
    }
  }

 public:
  long long countPaths(int n, const vector<vector<int>>& edges) {
    fill(begin(isPrime), end(isPrime), true);
    isPrime[1] = false;
    for (int i = 2; i * i <= n; i++) {
      if (isPrime[i]) {
        for (int j = i * i; j <= n; j += i) {
          isPrime[j] = false;
        }
      }
    }

    vector<vector<int>> G(n + 1);
    for (const auto& edge : edges) {
      int i = edge[0];
      int j = edge[1];
      G[i].push_back(j);
      G[j].push_back(i);
    }

    vector<int> seen;
    long long res = 0;
    vector<long long> count(n + 1, 0);
    for (int i = 1; i <= n; i++) {
      if (!isPrime[i]) {
        continue;
      }
      long long cur = 0;
      for (int j : G[i]) {
        if (isPrime[j]) {
          continue;
        }
        if (count[j] == 0) {
          seen.clear();
          dfs(G, seen, j, 0);
          long long cnt = seen.size();
          for (int k : seen) {
            count[k] = cnt;
          }
        }
        res += count[j] * cur;
        cur += count[j];
      }
      res += cur;
    }
    return res;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, long long>> tests{
      {5, {{1, 2}, {1, 3}, {2, 4}, {2, 5}}, 4},
      {6, {{1, 2}, {1, 3}, {2, 4}, {3, 5}, {3, 6}}, 6},
  };

  for (auto& [n, edges, ans] : tests) {
    assert(Solution().countPaths(n, edges) == ans);
  }
}