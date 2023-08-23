/*
 * @Date: 2023-08-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-23
 * @FilePath: /algorithm/cpp/1782_count_pairs/count_pairs.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> countPairs(int n, vector<vector<int>>& edges, vector<int>& queries) {
    vector<int> degree(n);
    unordered_map<int, int> cnt;
    for (auto edge : edges) {
      int x = edge[0] - 1, y = edge[1] - 1;
      if (x > y) {
        swap(x, y);
      }
      degree[x]++;
      degree[y]++;
      cnt[x * n + y]++;
    }

    vector<int> arr = degree;
    vector<int> ans;
    sort(arr.begin(), arr.end());
    for (int bound : queries) {
      int total = 0;
      for (int i = 0, j = n - 1; i < n; i++) {
        while (j > i && arr[i] + arr[j] > bound) {
          j--;
        }
        total += n - 1 - max(i, j);
      }
      for (auto& [val, freq] : cnt) {
        int x = val / n;
        int y = val % n;
        if (degree[x] + degree[y] > bound && degree[x] + degree[y] - freq <= bound) {
          total--;
        }
      }
      ans.emplace_back(total);
    }

    return ans;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>, vector<int>>> test_cases{
      {4, {{1, 2}, {2, 4}, {1, 3}, {2, 3}, {2, 1}}, {2, 3}, {6, 5}},
      {5, {{1, 5}, {1, 5}, {3, 4}, {2, 5}, {1, 3}, {5, 1}, {2, 3}, {2, 5}}, {1, 2, 3, 4, 5}, {10, 10, 9, 8, 6}},
  };

  for (auto& [n, edges, queries, ans] : test_cases) {
    assert(Solution().countPairs(n, edges, queries) == ans);
  }
}
