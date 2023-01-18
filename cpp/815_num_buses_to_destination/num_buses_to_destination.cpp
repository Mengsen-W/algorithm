/*
 * @Date: 2021-06-28 08:36:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-28 08:54:41
 */

#include <cassert>
#include <queue>
#include <unordered_map>
#include <vector>

using namespace std;

int numBusesToDestination(vector<vector<int>>& routes, int source, int target) {
  if (source == target) return 0;
  unordered_map<int, vector<int>> stations;
  int n = routes.size();
  vector<vector<int>> eg(n);

  for (int i = 0; i < n; ++i) {
    for (auto& r : routes[i]) {
      for (auto& s : stations[r]) {
        eg[s].push_back(i);
        eg[i].push_back(s);
      }
      stations[r].push_back(i);
    }
  }
  vector<int> vis(n);
  queue<int> q;
  for (auto& s : stations[source]) {
    q.push(s);
    vis[s] = 1;
  }
  for (auto& s : stations[target]) {
    if (vis[s] == 1) return 1;
    q.push(s);
    vis[s] = 2;
  }
  int qs, cnt = 1;
  while ((qs = q.size())) {
    if (vis[q.front()] == 2 || vis[q.back()] == 1) return -1;
    for (int i = 0; i < qs; ++i) {
      int now = q.front();
      for (auto& next : eg[now]) {
        if ((vis[next] | vis[now]) == 3) {
          return cnt + vis[now];
        }
        if (!vis[next]) {
          q.push(next);
          vis[next] = vis[now];
        }
      }
      q.pop();
    }
    cnt += 2;
  }
  return -1;
}

int main() {
  {
    vector<vector<int>> routes{{1, 2, 7}, {3, 6, 7}};
    int source = 1;
    int target = 6;
    assert(numBusesToDestination(routes, source, target) == 2);
  }
  {
    vector<vector<int>> routes{{7, 12}, {4, 5, 15}, {6}, {15, 19}, {9, 12, 13}};
    int source = 15;
    int target = 12;
    assert(numBusesToDestination(routes, source, target) == -1);
  }
}