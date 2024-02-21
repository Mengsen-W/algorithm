/*
 * @Date: 2024-02-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-21
 * @FilePath: /algorithm/cpp/106_build_tree/build_tree.cpp
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
#include <stack>
#include <vector>

using namespace std;

class Solution {
 public:
  TreeNode *buildTree(vector<int> &inorder, vector<int> &postorder) {
    if (postorder.size() == 0) {
      return nullptr;
    }
    auto root = new TreeNode(postorder[postorder.size() - 1]);
    auto s = stack<TreeNode *>();
    s.push(root);
    int inorderIndex = inorder.size() - 1;
    for (int i = int(postorder.size()) - 2; i >= 0; i--) {
      int postorderVal = postorder[i];
      auto node = s.top();
      if (node->val != inorder[inorderIndex]) {
        node->right = new TreeNode(postorderVal);
        s.push(node->right);
      } else {
        while (!s.empty() && s.top()->val == inorder[inorderIndex]) {
          node = s.top();
          s.pop();
          inorderIndex--;
        }
        node->left = new TreeNode(postorderVal);
        s.push(node->left);
      }
    }
    return root;
  }
};

bool isSameTree(TreeNode *p, TreeNode *q) {
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

int main() {
  vector<tuple<vector<int>, vector<int>, TreeNode *>> tests{
      {{9, 3, 15, 20, 7},
       {9, 15, 7, 20, 3},
       new TreeNode{3, new TreeNode{9}, new TreeNode{20, new TreeNode{15}, new TreeNode{7}}}},
      {{-1}, {-1}, new TreeNode{-1}},
  };

  for (auto &[preorder, inorder, root] : tests) {
    assert(isSameTree(Solution().buildTree(preorder, inorder), root) == true);
  }
}