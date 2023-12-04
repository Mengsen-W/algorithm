/*
 * @Date: 2023-12-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-04
 * @FilePath: /algorithm/cpp/1038_bst_to_gst/bst_to_gst.cpp
 */

// Definition for a binary tree node.
#include <cassert>
#include <cstddef>
#include <tuple>
#include <vector>
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

bool isSameTree(TreeNode* p, TreeNode* q) {
  if (p == nullptr && q == nullptr) {
    return true;
  } else if (p == nullptr || q == nullptr) {
    return false;
  } else if (p->val != q->val) {
    return false;
  } else {
    return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
  }
}

class Solution {
 public:
  TreeNode* getSuccessor(TreeNode* node) {
    TreeNode* succ = node->right;
    while (succ->left != nullptr && succ->left != node) {
      succ = succ->left;
    }
    return succ;
  }

  TreeNode* bstToGst(TreeNode* root) {
    int sum = 0;
    TreeNode* node = root;

    while (node != nullptr) {
      if (node->right == nullptr) {
        sum += node->val;
        node->val = sum;
        node = node->left;
      } else {
        TreeNode* succ = getSuccessor(node);
        if (succ->left == nullptr) {
          succ->left = node;
          node = node->right;
        } else {
          succ->left = nullptr;
          sum += node->val;
          node->val = sum;
          node = node->left;
        }
      }
    }

    return root;
  }
};

int main() {
  std::vector<std::tuple<TreeNode*, TreeNode*>> tests{
      {
          new TreeNode(4, new TreeNode(1, new TreeNode(0), new TreeNode(2, nullptr, new TreeNode(3))),
                       new TreeNode(6, new TreeNode(5), new TreeNode(7, nullptr, new TreeNode(8)))),
          new TreeNode(30, new TreeNode(36, new TreeNode(36), new TreeNode(35, nullptr, new TreeNode(33))),
                       new TreeNode(21, new TreeNode(26), new TreeNode(15, nullptr, new TreeNode(8)))),
      },
      {
          new TreeNode(0, nullptr, new TreeNode(1)),
          new TreeNode(1, nullptr, new TreeNode(1)),
      },
  };

  for (auto& [root, ans] : tests) {
    assert(isSameTree(Solution().bstToGst(root), ans));
  }
}