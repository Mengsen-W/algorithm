/*
 * @Date: 2022-08-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-05
 * @FilePath: /algorithm/623_add_one_row/add_one_row.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

//  Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
 public:
  TreeNode *addOneRow(TreeNode *root, int val, int depth) {
    if (depth == 1) {
      return new TreeNode(val, root, nullptr);
    }
    vector<TreeNode *> curLevel(1, root);
    for (int i = 1; i < depth - 1; i++) {
      vector<TreeNode *> tmpt;
      for (auto &node : curLevel) {
        if (node->left != nullptr) {
          tmpt.emplace_back(node->left);
        }
        if (node->right != nullptr) {
          tmpt.emplace_back(node->right);
        }
      }
      curLevel = move(tmpt);
    }
    for (auto &node : curLevel) {
      node->left = new TreeNode(val, node->left, nullptr);
      node->right = new TreeNode(val, nullptr, node->right);
    }
    return root;
  }
};

bool isSameTree(TreeNode *p, TreeNode *q) {
  if (p == nullptr || q == nullptr) {
    return p == q;
  }

  if (p->val != q->val) {
    return false;
  }

  return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
}

int main() {
  {
    TreeNode *root =
        new TreeNode{4, new TreeNode{2, new TreeNode{3}, new TreeNode{1}}, new TreeNode{6, new TreeNode{5}, nullptr}};
    int val = 1;
    int depth = 2;
    TreeNode *ans = new TreeNode{4, new TreeNode{1, new TreeNode{2, new TreeNode{3}, new TreeNode{1}}, nullptr},
                                 new TreeNode{1, nullptr, new TreeNode{6, new TreeNode{5}, nullptr}}};
    assert(isSameTree(Solution().addOneRow(root, val, depth), ans));
  }

  {
    TreeNode *root = new TreeNode{4, new TreeNode{2, new TreeNode{3}, new TreeNode{1}}, nullptr};
    int val = 1;
    int depth = 3;
    TreeNode *ans = new TreeNode{
        4, new TreeNode{2, new TreeNode{1, new TreeNode{3}, nullptr}, new TreeNode{1, nullptr, new TreeNode{1}}},
        nullptr};
    assert(isSameTree(Solution().addOneRow(root, val, depth), ans));
  }
}