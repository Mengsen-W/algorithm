/*
 * @Date: 2024-03-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-26
 * @FilePath: /algorithm/cpp/2642_Graph/Graph.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Graph {
 public:
  Graph(int n, vector<vector<int>>& edges) {
    dist = vector<vector<int>>(n, vector<int>(n, INT_MAX));
    for (int i = 0; i < n; i++) {
      dist[i][i] = 0;
    }
    for (auto& e : edges) {
      dist[e[0]][e[1]] = e[2];
    }
    for (int k = 0; k < n; k++) {
      for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
          if (dist[i][k] != INT_MAX && dist[k][j] != INT_MAX) {
            dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
          }
        }
      }
    }
  }

  void addEdge(vector<int> edge) {
    int x = edge[0], y = edge[1], cost = edge[2];
    if (cost >= dist[x][y]) {
      return;
    }
    int n = dist.size();
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < n; j++) {
        if (dist[i][x] != INT_MAX && dist[y][j] != INT_MAX) {
          dist[i][j] = min(dist[i][j], dist[i][x] + cost + dist[y][j]);
        }
      }
    }
  }

  int shortestPath(int node1, int node2) {
    int res = dist[node1][node2];
    return res == INT_MAX ? -1 : res;
  }

 private:
  vector<vector<int>> dist;
};

int main() {
  vector<vector<int>> graph{{0, 2, 5}, {0, 1, 2}, {1, 2, 1}, {3, 0, 3}};
  Graph g = Graph(4, graph);
  assert(g.shortestPath(3, 2) == 6);
  // 返回 6 。从 3 到 2 的最短路径如第一幅图所示：3 -> 0 -> 1 -> 2 ，总代价为 3 + 2 + 1 = 6 。
  assert(g.shortestPath(0, 3) == -1);  // 返回 -1 。没有从 0 到 3 的路径。
  g.addEdge({1, 3, 4});                // 添加一条节点 1 到节点 3 的边，得到第二幅图。
  assert(g.shortestPath(0, 3) == 6);  // 返回 6 。从 0 到 3 的最短路径为 0 -> 1 -> 3 ，总代价为 2 + 4 = 6 。
}