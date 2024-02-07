/*
 * @Date: 2024-02-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-07
 * @FilePath: /algorithm/cpp/2641_replace_value_in_tree/replace_value_in_tree.cpp
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
  TreeNode *replaceValueInTree(TreeNode *root) {
    vector<TreeNode *> q = {root};
    root->val = 0;
    while (!q.empty()) {
      vector<TreeNode *> q2;
      int sum = 0;
      for (auto fa : q) {
        if (fa->left) {
          q2.push_back(fa->left);
          sum += fa->left->val;
        }
        if (fa->right) {
          q2.push_back(fa->right);
          sum += fa->right->val;
        }
      }
      for (auto fa : q) {
        int child_sum = (fa->left ? fa->left->val : 0) + (fa->right ? fa->right->val : 0);
        if (fa->left) {
          fa->left->val = sum - child_sum;
        }
        if (fa->right) {
          fa->right->val = sum - child_sum;
        }
      }
      q = std::move(q2);
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
  vector<tuple<TreeNode *, TreeNode *>> tests{
      {
          new TreeNode{5, new TreeNode{4, new TreeNode{1}, new TreeNode{10}},
                       new TreeNode{9, nullptr, new TreeNode{7}}},
          new TreeNode{0, new TreeNode{0, new TreeNode{7}, new TreeNode{7}},
                       new TreeNode{0, nullptr, new TreeNode{11}}},
      },
      {
          new TreeNode{3, new TreeNode{1}, new TreeNode{2}},
          new TreeNode{0, new TreeNode{0}, new TreeNode{0}},
      },
  };

  for (auto &[root, ans] : tests) {
    assert(isSameTree(Solution().replaceValueInTree(root), ans) == true);
  }
}