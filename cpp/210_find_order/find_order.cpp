/*
 * @Date: 2023-09-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-10
 * @FilePath: /algorithm/cpp/210_find_order/find_order.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 private:
  // 存储有向图
  vector<vector<int>> edges;
  // 存储每个节点的入度
  vector<int> indeg;
  // 存储答案
  vector<int> result;

 public:
  vector<int> findOrder(int numCourses, vector<vector<int>>& prerequisites) {
    edges.resize(numCourses);
    indeg.resize(numCourses);
    for (const auto& info : prerequisites) {
      edges[info[1]].push_back(info[0]);
      ++indeg[info[0]];
    }

    queue<int> q;
    // 将所有入度为 0 的节点放入队列中
    for (int i = 0; i < numCourses; ++i) {
      if (indeg[i] == 0) {
        q.push(i);
      }
    }

    while (!q.empty()) {
      // 从队首取出一个节点
      int u = q.front();
      q.pop();
      // 放入答案中
      result.push_back(u);
      for (int v : edges[u]) {
        --indeg[v];
        // 如果相邻节点 v 的入度为 0，就可以选 v 对应的课程了
        if (indeg[v] == 0) {
          q.push(v);
        }
      }
    }

    if (result.size() != numCourses) {
      return {};
    }
    return result;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>>> tests{
      {2, {{1, 0}}, {0, 1}},
      {1, {}, {0}},
  };

  for (auto& [numCourses, prerequisites, expected] : tests) {
    assert(Solution().findOrder(numCourses, prerequisites) == expected);
  }
}