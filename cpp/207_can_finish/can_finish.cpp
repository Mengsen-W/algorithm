/*
 * @Date: 2023-09-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-09
 * @FilePath: /algorithm/cpp/207_can_finish/can_finish.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 private:
  vector<vector<int>> edges;
  vector<int> indeg;

 public:
  bool canFinish(int numCourses, vector<vector<int>>& prerequisites) {
    edges.resize(numCourses);
    indeg.resize(numCourses);
    for (const auto& info : prerequisites) {
      edges[info[1]].push_back(info[0]);
      ++indeg[info[0]];
    }

    queue<int> q;
    for (int i = 0; i < numCourses; ++i) {
      if (indeg[i] == 0) {
        q.push(i);
      }
    }

    int visited = 0;
    while (!q.empty()) {
      ++visited;
      int u = q.front();
      q.pop();
      for (int v : edges[u]) {
        --indeg[v];
        if (indeg[v] == 0) {
          q.push(v);
        }
      }
    }

    return visited == numCourses;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, bool>> tests{
      {2, {{1, 0}}, true},
      {2, {{1, 0}, {0, 1}}, false},
  };
  for (auto& [numCourses, prerequisites, expected] : tests) {
    assert(Solution().canFinish(numCourses, prerequisites) == expected);
  }
}
