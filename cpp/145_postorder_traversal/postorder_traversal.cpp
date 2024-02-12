/*
 * @Date: 2024-02-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-12
 * @FilePath: /algorithm/cpp/145_postorder_traversal/postorder_traversal.cpp
 */

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  void postorder(TreeNode *root, vector<int> &res) {
    if (root == nullptr) {
      return;
    }
    postorder(root->left, res);
    postorder(root->right, res);
    res.push_back(root->val);
  }

  vector<int> postorderTraversal(TreeNode *root) {
    vector<int> res;
    postorder(root, res);
    return res;
  }
};

int main() {
  vector<tuple<TreeNode *, vector<int>>> tests{
      {new TreeNode{1, nullptr, new TreeNode{2, new TreeNode{3}, nullptr}}, {3, 2, 1}},
      {nullptr, {}},
      {new TreeNode{1}, {1}},
  };

  for (auto &[root, ans] : tests) {
    assert(Solution().postorderTraversal(root) == ans);
  }
}