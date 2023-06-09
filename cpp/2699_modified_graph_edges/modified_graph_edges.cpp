/*
 * @Date: 2023-06-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-09
 * @FilePath: /algorithm/cpp/2699_modified_graph_edges/modified_graph_edges.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> modifiedGraphEdges(int n, vector<vector<int>>& edges, int source, int destination, int target) {
    this->target = target;
    vector<vector<int>> adj_matrix(n, vector<int>(n, -1));
    // 邻接矩阵中存储边的下标
    for (int i = 0; i < edges.size(); ++i) {
      int u = edges[i][0], v = edges[i][1];
      adj_matrix[u][v] = adj_matrix[v][u] = i;
    }
    from_destination = dijkstra(0, destination, edges, adj_matrix);
    if (from_destination[source] > target) {
      return {};
    }
    vector<long long> from_source = dijkstra(1, source, edges, adj_matrix);
    if (from_source[destination] != target) {
      return {};
    }
    return edges;
  }

  vector<long long> dijkstra(int op, int source, vector<vector<int>>& edges, const vector<vector<int>>& adj_matrix) {
    // 朴素的 dijkstra 算法
    // adj_matrix 是一个邻接矩阵
    int n = adj_matrix.size();
    vector<long long> dist(n, INT_MAX / 2);
    vector<int> used(n);
    dist[source] = 0;

    for (int round = 0; round < n - 1; ++round) {
      int u = -1;
      for (int i = 0; i < n; ++i) {
        if (!used[i] && (u == -1 || dist[i] < dist[u])) {
          u = i;
        }
      }
      used[u] = true;
      for (int v = 0; v < n; ++v) {
        if (!used[v] && adj_matrix[u][v] != -1) {
          if (edges[adj_matrix[u][v]][2] != -1) {
            dist[v] = min(dist[v], dist[u] + edges[adj_matrix[u][v]][2]);
          } else {
            if (op == 0) {
              dist[v] = min(dist[v], dist[u] + 1);
            } else {
              int modify = target - dist[u] - from_destination[v];
              if (modify > 0) {
                dist[v] = min(dist[v], dist[u] + modify);
                edges[adj_matrix[u][v]][2] = modify;
              } else {
                edges[adj_matrix[u][v]][2] = target;
              }
            }
          }
        }
      }
    }

    return dist;
  }

 private:
  vector<long long> from_destination;
  int target;
};

int main() {
  {
    int n = 5;
    vector<vector<int>> edges{{4, 1, -1}, {2, 0, -1}, {0, 3, -1}, {4, 3, -1}};
    int source = 0;
    int destination = 1;
    int target = 5;
    vector<vector<int>> ans{{4, 1, 1}, {2, 0, 1}, {0, 3, 3}, {4, 3, 1}};
    assert(Solution().modifiedGraphEdges(n, edges, source, destination, target) == ans);
  }

  {
    int n = 3;
    vector<vector<int>> edges{{0, 1, -1}, {0, 2, 5}};
    int source = 0;
    int destination = 2;
    int target = 6;
    vector<vector<int>> ans{};
    assert(Solution().modifiedGraphEdges(n, edges, source, destination, target) == ans);
  }

  {
    int n = 4;
    vector<vector<int>> edges{{1, 0, 4}, {1, 2, 3}, {2, 3, 5}, {0, 3, -1}};
    int source = 0;
    int destination = 2;
    int target = 6;
    vector<vector<int>> ans{{1, 0, 4}, {1, 2, 3}, {2, 3, 5}, {0, 3, 1}};
    assert(Solution().modifiedGraphEdges(n, edges, source, destination, target) == ans);
  }
}