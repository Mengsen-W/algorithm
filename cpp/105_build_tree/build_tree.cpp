/*
 * @Date: 2024-02-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-20
 * @FilePath: /algorithm/cpp/105_build_tree/build_tree.cpp
 */

// Definition for a binary tree node.
#include <cassert>
#include <tuple>
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

#include <stack>
#include <vector>

using namespace std;

class Solution {
 public:
  TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
    if (!preorder.size()) {
      return nullptr;
    }
    TreeNode* root = new TreeNode(preorder[0]);
    stack<TreeNode*> stk;
    stk.push(root);
    int inorderIndex = 0;
    for (int i = 1; i < preorder.size(); ++i) {
      int preorderVal = preorder[i];
      TreeNode* node = stk.top();
      if (node->val != inorder[inorderIndex]) {
        node->left = new TreeNode(preorderVal);
        stk.push(node->left);
      } else {
        while (!stk.empty() && stk.top()->val == inorder[inorderIndex]) {
          node = stk.top();
          stk.pop();
          ++inorderIndex;
        }
        node->right = new TreeNode(preorderVal);
        stk.push(node->right);
      }
    }
    return root;
  }
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

int main() {
  vector<tuple<vector<int>, vector<int>, TreeNode*>> tests{
      {{3, 9, 20, 15, 7},
       {9, 3, 15, 20, 7},
       new TreeNode{3, new TreeNode{9}, new TreeNode{20, new TreeNode{15}, new TreeNode{7}}}},
      {{-1}, {-1}, new TreeNode{-1}},
  };

  for (auto& [preorder, inorder, root] : tests) {
    assert(isSameTree(Solution().buildTree(preorder, inorder), root) == true);
  }
}
