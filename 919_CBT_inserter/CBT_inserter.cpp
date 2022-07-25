/*
 * @Date: 2022-07-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-25
 * @FilePath: /algorithm/919_CBT_inserter/CBT_inserter.cpp
 */

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

#include <cassert>
#include <queue>

using namespace std;

class CBTInserter {
 public:
  CBTInserter(TreeNode* root) {
    this->root = root;

    queue<TreeNode*> q;
    q.push(root);

    while (!q.empty()) {
      ++cnt;
      TreeNode* node = q.front();
      q.pop();
      if (node->left) {
        q.push(node->left);
      }
      if (node->right) {
        q.push(node->right);
      }
    }
  }

  int insert(int val) {
    ++cnt;
    TreeNode* child = new TreeNode(val);
    TreeNode* node = root;
    int highbit = 31 - __builtin_clz(cnt);
    for (int i = highbit - 1; i >= 1; --i) {
      if (cnt & (1 << i)) {
        node = node->right;
      } else {
        node = node->left;
      }
    }
    if (cnt & 1) {
      node->right = child;
    } else {
      node->left = child;
    }
    return node->val;
  }

  TreeNode* get_root() { return root; }

 private:
  int cnt = 0;
  TreeNode* root;
};

int main() {
  TreeNode* root = new TreeNode(1, new TreeNode(2), nullptr);
  CBTInserter c{root};
  assert(c.insert(3) == 1);
  assert(c.insert(4) == 2);
  assert(c.get_root()->val == root->val);
}