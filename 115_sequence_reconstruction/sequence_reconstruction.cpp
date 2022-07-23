/*
 * @Date: 2022-07-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-23
 * @FilePath: /algorithm/115_sequence_reconstruction/sequence_reconstruction.cpp
 */

#include <cassert>
#include <queue>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  bool sequenceReconstruction(vector<int>& nums, vector<vector<int>>& sequences) {
    int n = nums.size();
    vector<int> indegrees(n + 1);
    vector<unordered_set<int>> graph(n + 1);
    for (auto& sequence : sequences) {
      for (int i = 1; i < sequence.size(); i++) {
        int prev = sequence[i - 1], next = sequence[i];
        if (!graph[prev].count(next)) {
          graph[prev].emplace(next);
          indegrees[next]++;
        }
      }
    }
    queue<int> qu;
    for (int i = 1; i <= n; i++) {
      if (indegrees[i] == 0) {
        qu.emplace(i);
      }
    }
    while (!qu.empty()) {
      if (qu.size() > 1) {
        return false;
      }
      int num = qu.front();
      qu.pop();
      for (int next : graph[num]) {
        indegrees[next]--;
        if (indegrees[next] == 0) {
          qu.emplace(next);
        }
      }
    }
    return true;
  }
};

int main() {
  {
    vector<int> nums{1, 2, 3};
    vector<vector<int>> sequences{{1, 2}, {1, 3}};
    bool ans = false;
    assert(Solution().sequenceReconstruction(nums, sequences) == ans);
  }

  {
    vector<int> nums{1, 2, 3};
    vector<vector<int>> sequences{{1, 2}};
    bool ans = false;
    assert(Solution().sequenceReconstruction(nums, sequences) == ans);
  }

  {
    vector<int> nums{1, 2, 3};
    vector<vector<int>> sequences{{1, 2}, {1, 3}, {2, 3}};
    bool ans = true;
    assert(Solution().sequenceReconstruction(nums, sequences) == ans);
  }
}
