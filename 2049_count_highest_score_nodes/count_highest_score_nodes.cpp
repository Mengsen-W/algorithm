/*
 * @Date: 2022-03-10 23:48:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-11 00:39:18
 * @FilePath: /algorithm/2049_count_highest_score_nodes/count_highest_score_nodes.cpp
 * /algorithm/2049_count_highest_score_nodes/count_highest_score_nodes.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  long maxScore = 0;
  int cnt = 0;
  int n;
  vector<vector<int>> children;

  int dfs(int node) {
    long score = 1;
    int size = n - 1;
    for (int c : children[node]) {
      int t = dfs(c);
      score *= t;
      size -= t;
    }
    if (node != 0) {
      score *= size;
    }
    if (score == maxScore) {
      cnt++;
    } else if (score > maxScore) {
      maxScore = score;
      cnt = 1;
    }
    return n - size;
  }

  int countHighestScoreNodes(vector<int>&& parents) {
    this->n = parents.size();
    this->children = vector<vector<int>>(n);
    for (int i = 0; i < n; i++) {
      int p = parents[i];
      if (p != -1) {
        children[p].emplace_back(i);
      }
    }
    dfs(0);
    return cnt;
  }
};

int main() {
  assert(Solution().countHighestScoreNodes({-1, 2, 0, 2, 0}) == 3);
  assert(Solution().countHighestScoreNodes({-1, 2, 0}) == 2);
}