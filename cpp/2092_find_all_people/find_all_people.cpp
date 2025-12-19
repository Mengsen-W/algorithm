#include <algorithm>
#include <cassert>
#include <queue>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findAllPeople(int n, vector<vector<int>>& meetings, int firstPerson) {
    int m = meetings.size();
    sort(meetings.begin(), meetings.end(), [&](const auto& x, const auto& y) { return x[2] < y[2]; });

    vector<int> secret(n);
    secret[0] = secret[firstPerson] = true;

    unordered_set<int> vertices;
    unordered_map<int, vector<int>> edges;

    for (int i = 0; i < m;) {
      // meetings[i .. j] 为同一时间
      int j = i;
      while (j + 1 < m && meetings[j + 1][2] == meetings[i][2]) {
        ++j;
      }

      vertices.clear();
      edges.clear();
      for (int k = i; k <= j; ++k) {
        int x = meetings[k][0], y = meetings[k][1];
        vertices.insert(x);
        vertices.insert(y);
        edges[x].push_back(y);
        edges[y].push_back(x);
      }

      queue<int> q;
      for (int u : vertices) {
        if (secret[u]) {
          q.push(u);
        }
      }
      while (!q.empty()) {
        int u = q.front();
        q.pop();
        for (int v : edges[u]) {
          if (!secret[v]) {
            secret[v] = true;
            q.push(v);
          }
        }
      }

      i = j + 1;
    }

    vector<int> ans;
    for (int i = 0; i < n; ++i) {
      if (secret[i]) {
        ans.push_back(i);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int, vector<int>>> tests{
      {6, {{1, 2, 5}, {2, 3, 8}, {1, 5, 10}}, 1, {0, 1, 2, 3, 5}},
      {4, {{3, 1, 3}, {1, 2, 2}, {0, 3, 3}}, 3, {0, 1, 3}},
      {5, {{3, 4, 2}, {1, 2, 1}, {2, 3, 1}}, 1, {0, 1, 2, 3, 4}},
  };

  for (auto& [n, meetings, firstPerson, expect] : tests) {
    assert(Solution().findAllPeople(n, meetings, firstPerson) == expect);
  }
}