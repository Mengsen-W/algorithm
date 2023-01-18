/*
 * @Date: 2021-10-17 09:40:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-17 09:45:29
 */

#include <cassert>
#include <unordered_map>
using namespace std;

struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right)
      : val(x), left(left), right(right) {}
};

class Solution {
 private:
  unordered_map<TreeNode*, int> leftchilds;
  unordered_map<TreeNode*, int> rightchilds;

 public:
  int myKthSmallest(TreeNode* root, int k) {
    TreeNode* cur = root;
    int rank = leftchilds[cur] + 1;
    while (k != rank) {
      if (k < rank) {
        cur = cur->left;
        rank -= rightchilds[cur] + 1;
      } else {
        cur = cur->right;
        rank += leftchilds[cur] + 1;
      }
    }
    return cur->val;
  }

  int memoTree(TreeNode* root) {
    if (!root) return 0;

    leftchilds[root] = memoTree(root->left);
    rightchilds[root] = memoTree(root->right);
    return leftchilds[root] + rightchilds[root] + 1;
  }

  int kthSmallest(TreeNode* root, int k) {
    if (!root) return 0;

    memoTree(root);
    return myKthSmallest(root, k);
  }
};

int main() {
  {
    TreeNode* root = new TreeNode{3, new TreeNode{1, nullptr, new TreeNode{2}},  new TreeNode{4}};
    assert(Solution().kthSmallest(root, 1) == 1);
  }
  {
    TreeNode* root = new TreeNode{5, new TreeNode{3, new TreeNode{2, new TreeNode{1}, nullptr}, new TreeNode{4}},  new TreeNode{6}};
    assert(Solution().kthSmallest(root, 3) == 3);
  }
}