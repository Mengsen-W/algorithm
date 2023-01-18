/*
 * @Date: 2021-04-25 09:51:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-25 10:21:09
 */
#include <iostream>

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

TreeNode *res_node;

void inorder(TreeNode *node) {
  if (node == nullptr) return;
  inorder(node->left);

  res_node->right = node;
  node->left = nullptr;
  res_node = node;

  inorder(node->right);
}

TreeNode *increasing_BST(TreeNode *root) {
  TreeNode *dummyNode = new TreeNode(-1);
  res_node = dummyNode;
  inorder(root);
  return dummyNode->right;
}

void print_BST(TreeNode *root) {
  if (root->left == nullptr && root->right == nullptr) {
    std::cout << root->val << std::endl;
    return;
  }
  std::cout << root->val << ", ";
  if (root->left == nullptr) {
    std::cout << "nullptr"
              << ", ";
  }
  print_BST(root->right);
}

int main() {
  {
    TreeNode *root = new TreeNode{
        5,
        new TreeNode{
            3, new TreeNode{2, new TreeNode{1, nullptr, nullptr}, nullptr},
            new TreeNode{4, nullptr, nullptr}},
        new TreeNode{6, nullptr,
                     new TreeNode{8, new TreeNode{7, nullptr, nullptr},
                                  new TreeNode{9, nullptr, nullptr}}}};

    root = increasing_BST(root);
    print_BST(root);
  }
  {
    TreeNode *root = new TreeNode{5, new TreeNode{1, nullptr, nullptr},
                                  new TreeNode{7, nullptr, nullptr}};

    root = increasing_BST(root);
    print_BST(root);
  }
  return 0;
}