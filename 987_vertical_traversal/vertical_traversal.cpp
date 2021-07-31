/*
 * @Date: 2021-07-31 00:49:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-31 01:16:07
 */

#include <algorithm>
#include <cassert>
#include <climits>
#include <functional>
#include <vector>

using namespace std;

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

class Solution {
 public:
  vector<vector<int>> verticalTraversal(TreeNode *root) {
    vector<tuple<int, int, int>> nodes;

    function<void(TreeNode *, int, int)> dfs = [&](TreeNode *node, int row,
                                                   int col) {
      if (!node) {
        return;
      }
      nodes.emplace_back(col, row, node->val);
      dfs(node->left, row + 1, col - 1);
      dfs(node->right, row + 1, col + 1);
    };

    dfs(root, 0, 0);
    sort(nodes.begin(), nodes.end());
    vector<vector<int>> ans;
    int lastcol = INT_MIN;
    for (const auto &[col, row, value] : nodes) {
      if (col != lastcol) {
        lastcol = col;
        ans.emplace_back();
      }
      ans.back().push_back(value);
    }
    return ans;
  }
};

int main() {
  {
    TreeNode *root =
        new TreeNode{3, new TreeNode{9},
                     new TreeNode{20, new TreeNode{15}, new TreeNode{7}}};
    vector<vector<int>> ans{{9}, {3, 15}, {20}, {7}};
    assert(Solution{}.verticalTraversal(root) == ans);
  }

  {
    TreeNode *root =
        new TreeNode{1, new TreeNode{2, new TreeNode{4}, new TreeNode{5}},
                     new TreeNode{3, new TreeNode{6}, new TreeNode{7}}};
    vector<vector<int>> ans{{4}, {2}, {1, 5, 6}, {3}, {7}};
    assert(Solution{}.verticalTraversal(root) == ans);
  }

    {
    TreeNode *root =
        new TreeNode{1, new TreeNode{2, new TreeNode{4}, new TreeNode{6}},
                     new TreeNode{3, new TreeNode{5}, new TreeNode{7}}};
    vector<vector<int>> ans{{4}, {2}, {1, 5, 6}, {3}, {7}};
    assert(Solution{}.verticalTraversal(root) == ans);
  }

  return 0;
}