/*
 * @Date: 2021-08-02 09:46:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-02 14:39:37
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

//迪杰特斯拉算法
int networkDelayTime_Dijkstra(vector<vector<int>>& times, int n, int k) {
  vector<vector<int>> arcs(n + 1, vector<int>(n + 1, 10001));
  //距离矩阵，10001表不可达
  for (int i = 1; i <= n; i++) {
    //初始化arcs矩阵到本节点的距离为0
    arcs[i][i] = 0;
  }
  for (const auto& time : times) {
    //按所给信息初始化arcs矩阵
    arcs[time[0]][time[1]] = time[2];
  }
  vector<int> dist = arcs[k];
  //定义且初始化距离矩阵dist,dist[i]表示第k个节点到第i个节点的距离
  vector<bool> flag(n + 1, false);
  //标记数组，flag[i]标记第i个节点是否确定
  flag[k] = true;
  for (int i = 1; i <= n; i++) {
    //共有n轮，每轮确定一个节点
    int minn = 10010;
    int pos;
    for (int j = 1; j <= n; j++) {
      //找到该轮中未确定且距离k节点最小的节点位置pos
      if (!flag[j] && dist[j] < minn) {
        pos = j;
        minn = dist[j];
      }
    }
    flag[pos] = true;  //将pos所在位置节点标记为已确定
    for (int j = 1; j <= n; j++) {
      //从pos所在位置节点出发，修改dist
      if (!flag[j] && dist[pos] + arcs[pos][j] < dist[j]) {
        dist[j] = dist[pos] + arcs[pos][j];
      }
    }
  }
  int ret = *max_element(dist.begin() + 1, dist.end());
  return ret == 10001 ? -1 : ret;
}

//弗洛伊德算法
int networkDelayTime_floyd(vector<vector<int>>& times, int n, int k) {
  vector<vector<int>> A(n + 1, vector<int>(n + 1, 10010));
  //距离矩阵A[i][j]表示节点i到节点j的距离，10010表不可达
  for (int i = 1; i <= n; i++) {
    //初始化A矩阵到本节点的距离为0
    A[i][i] = 0;
  }
  for (const auto& time : times) {
    //按所给信息初始化A矩阵
    A[time[0]][time[1]] = time[2];
  }
  for (int r = 1; r <= n; r++) {
    //考虑以第r个节点作为中转点
    for (int i = 1; i <= n; i++) {
      //遍历整个矩阵，i为行，j为列
      for (int j = 1; j <= n; j++) {
        if (A[i][j] > A[i][r] + A[r][j]) {
          //若以r为中转点的路径更短，则更新路径长度
          A[i][j] = A[i][r] + A[r][j];
        }
      }
    }
  }
  int ret = *max_element(A[k].begin() + 1, A[k].end());
  return ret == 10010 ? -1 : ret;
}

int main() {
  {
    vector<vector<int>> times{{2, 1, 1}, {2, 3, 1}, {3, 4, 1}};
    int n = 4;
    int k = 2;
    int ans = 2;
    assert(networkDelayTime_Dijkstra(times, n, k) == ans);
    assert(networkDelayTime_floyd(times, n, k) == ans);
  }
  {
    vector<vector<int>> times{{1, 2, 1}};
    int n = 2;
    int k = 1;
    int ans = 1;
    assert(networkDelayTime_Dijkstra(times, n, k) == ans);
    assert(networkDelayTime_floyd(times, n, k) == ans);
  }
  {
    vector<vector<int>> times{{1, 2, 1}};
    int n = 2;
    int k = 2;
    int ans = -1;
    assert(networkDelayTime_Dijkstra(times, n, k) == ans);
    assert(networkDelayTime_floyd(times, n, k) == ans);
  }
}