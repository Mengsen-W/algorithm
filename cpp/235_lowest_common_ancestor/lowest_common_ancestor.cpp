/*
 * @Date: 2024-02-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-25
 * @FilePath: /algorithm/cpp/235_lowest_common_ancestor/lowest_common_ancestor.cpp
 */

// TreeNode Definition for a binary tree node.
#include <tuple>
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

#include <vector>

using namespace std;

class Solution {
 public:
  vector<TreeNode*> getPath(TreeNode* root, TreeNode* target) {
    vector<TreeNode*> path;
    TreeNode* node = root;
    while (node != target) {
      path.push_back(node);
      if (target->val < node->val) {
        node = node->left;
      } else {
        node = node->right;
      }
    }
    path.push_back(node);
    return path;
  }

  TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
    vector<TreeNode*> path_p = getPath(root, p);
    vector<TreeNode*> path_q = getPath(root, q);
    TreeNode* ancestor;
    for (int i = 0; i < path_p.size() && i < path_q.size(); ++i) {
      if (path_p[i] == path_q[i]) {
        ancestor = path_p[i];
      } else {
        break;
      }
    }
    return ancestor;
  }
};

int main() {}