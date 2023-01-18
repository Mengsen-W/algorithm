/*
 * @Date: 2021-07-28 17:36:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-28 17:50:11
 */

#include <cassert>
#include <unordered_map>
#include <vector>
using namespace std;

struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right)
      : val(x), left(left), right(right) {}
};

class Solution {
  unordered_map<int, TreeNode*> parents;
  vector<int> ans;

  void findParents(TreeNode* node) {
    if (node->left != nullptr) {
      parents[node->left->val] = node;
      findParents(node->left);
    }
    if (node->right != nullptr) {
      parents[node->right->val] = node;
      findParents(node->right);
    }
  }

  void findAns(TreeNode* node, TreeNode* from, int depth, int k) {
    if (node == nullptr) {
      return;
    }
    if (depth == k) {
      ans.push_back(node->val);
      return;
    }
    if (node->left != from) {
      findAns(node->left, node, depth + 1, k);
    }
    if (node->right != from) {
      findAns(node->right, node, depth + 1, k);
    }
    if (parents[node->val] != from) {
      findAns(parents[node->val], node, depth + 1, k);
    }
  }

 public:
  vector<int> distanceK(TreeNode* root, TreeNode* target, int k) {
    // 从 root 出发 DFS，记录每个结点的父结点
    findParents(root);

    // 从 target 出发 DFS，寻找所有深度为 k 的结点
    findAns(target, nullptr, 0, k);

    return ans;
  }
};

int main() {
  TreeNode* root = new TreeNode{
      3,
      new TreeNode{5, new TreeNode{6, nullptr, nullptr},
                   new TreeNode{2, new TreeNode{7, nullptr, nullptr},
                                new TreeNode{4, nullptr, nullptr}}},
      new TreeNode{1, new TreeNode{0, nullptr, nullptr},
                   new TreeNode{0, nullptr, nullptr}}};
  vector<int> ans{7, 4, 1};
  assert(Solution{}.distanceK(root, root->left, 2) == ans);
}
