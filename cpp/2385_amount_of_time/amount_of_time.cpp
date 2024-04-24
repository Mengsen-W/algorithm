/*
 * @Date: 2024-04-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-24
 * @FilePath: /algorithm/cpp/2385_amount_of_time/amount_of_time.cpp
 */

// Definition for a binary tree node.
#include <cassert>
#include <tuple>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <functional>
#include <queue>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int amountOfTime(TreeNode *root, int start) {
    unordered_map<int, vector<int>> graph;
    function<void(TreeNode *)> dfs = [&](TreeNode *node) {
      for (TreeNode *child : vector<TreeNode *>{node->left, node->right}) {
        if (child != nullptr) {
          graph[node->val].push_back(child->val);
          graph[child->val].push_back(node->val);
          dfs(child);
        }
      }
    };
    dfs(root);
    queue<vector<int>> q;
    q.push({start, 0});
    unordered_set<int> visited;
    visited.insert(start);
    int time = 0;
    while (!q.empty()) {
      auto arr = q.front();
      q.pop();
      int nodeVal = arr[0];
      time = arr[1];
      for (int childVal : graph[nodeVal]) {
        if (!visited.count(childVal)) {
          q.push({childVal, time + 1});
          visited.insert(childVal);
        }
      }
    }
    return time;
  }
};

int main() {
  vector<tuple<TreeNode *, int, int>> tests{
      {
          new TreeNode{1, new TreeNode{5, nullptr, new TreeNode{4, new TreeNode{9}, new TreeNode{2}}},
                       new TreeNode{3, new TreeNode{10}, new TreeNode{6}}},
          3,
          4,
      },
      {
          new TreeNode{1},
          1,
          0,
      },
  };

  for (auto &[root, start, ans] : tests) {
    assert(Solution().amountOfTime(root, start) == ans);
  }
}
