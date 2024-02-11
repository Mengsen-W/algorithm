/*
 * @Date: 2024-02-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-11
 * @FilePath: /algorithm/cpp/144_preorder_traversal/preorder_traversal.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

// Definition for a binary tree node.
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
  void preorder(TreeNode *root, vector<int> &res) {
    if (root == nullptr) {
      return;
    }
    res.push_back(root->val);
    preorder(root->left, res);
    preorder(root->right, res);
  }

  vector<int> preorderTraversal(TreeNode *root) {
    vector<int> res;
    preorder(root, res);
    return res;
  }
};

int main() {
  vector<tuple<TreeNode *, vector<int>>> tests{
      {new TreeNode{1, nullptr, new TreeNode{2, new TreeNode{3}, nullptr}}, {1, 2, 3}},
      {nullptr, {}},
      {new TreeNode{1, nullptr, new TreeNode{2}}, {1, 2}},
      {new TreeNode{1, new TreeNode{2}, nullptr}, {1, 2}},
  };

  for (auto &[root, ans] : tests) {
    assert(Solution().preorderTraversal(root) == ans);
  }
}