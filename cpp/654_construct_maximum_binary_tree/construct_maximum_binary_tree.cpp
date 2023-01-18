/*
 * @Date: 2022-08-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-20
 * @FilePath: /algorithm/654_construct_maximum_binary_tree/construct_maximum_binary_tree.cpp
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
  TreeNode *constructMaximumBinaryTree(vector<int> &nums) {
    int n = nums.size();
    vector<int> stk;
    vector<TreeNode *> tree(n);
    for (int i = 0; i < n; ++i) {
      tree[i] = new TreeNode(nums[i]);
      while (!stk.empty() && nums[i] > nums[stk.back()]) {
        tree[i]->left = tree[stk.back()];
        stk.pop_back();
      }
      if (!stk.empty()) {
        tree[stk.back()]->right = tree[i];
      }
      stk.push_back(i);
    }
    return tree[stk[0]];
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
  {
    vector<int> nums{3, 2, 1, 6, 0, 5};
    TreeNode *ans = new TreeNode(6, new TreeNode(3, nullptr, new TreeNode(2, nullptr, new TreeNode(1))),
                                 new TreeNode(5, new TreeNode(0), nullptr));
    assert(isSameTree(Solution().constructMaximumBinaryTree(nums), ans));
  }
  {
    vector<int> nums{3, 2, 1};
    TreeNode *ans = new TreeNode(3, nullptr, new TreeNode(2, nullptr, new TreeNode(1)));
    assert(isSameTree(Solution().constructMaximumBinaryTree(nums), ans));
  }
}
