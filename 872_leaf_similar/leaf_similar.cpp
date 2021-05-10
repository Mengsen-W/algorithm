/*
 * @Date: 2021-05-10 08:45:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-10 08:57:16
 */

#include <cassert>
#include <vector>

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

void dfs(TreeNode* node, vector<int>& seq) {
  if (!node->left && !node->right) {
    seq.push_back(node->val);
  } else {
    if (node->left) {
      dfs(node->left, seq);
    }
    if (node->right) {
      dfs(node->right, seq);
    }
  }
}

bool leafSimilar(TreeNode* root1, TreeNode* root2) {
  vector<int> seq1;
  if (root1) {
    dfs(root1, seq1);
  }

  vector<int> seq2;
  if (root2) {
    dfs(root2, seq2);
  }

  return seq1 == seq2;
}

int main() {
  {
    TreeNode* root1 = new TreeNode{
        3,
        new TreeNode{5, new TreeNode{6},
                     new TreeNode{2, new TreeNode{7}, new TreeNode{4}}},
        new TreeNode{1, new TreeNode{9}, new TreeNode{8}}};
    TreeNode* root2 = new TreeNode{
        3, new TreeNode{5, new TreeNode{6}, new TreeNode{7}},
        new TreeNode{1, new TreeNode{4},
                     new TreeNode{2, new TreeNode{9}, new TreeNode{8}}}};
    assert(leafSimilar(root1, root2));
  }
  {
    TreeNode* root1 = new TreeNode{1};
    TreeNode* root2 = new TreeNode{1};
    assert(leafSimilar(root1, root2));
  }
  {
    TreeNode* root1 = new TreeNode{1};
    TreeNode* root2 = new TreeNode{2};
    assert(!leafSimilar(root1, root2));
  }
  {
    TreeNode* root1 = new TreeNode{1, new TreeNode{2}, nullptr};
    TreeNode* root2 = new TreeNode{2, new TreeNode{2}, nullptr};
    assert(leafSimilar(root1, root2));
  }
  {
    TreeNode* root1 = new TreeNode{1, new TreeNode{2}, new TreeNode{3}};
    TreeNode* root2 = new TreeNode{2, new TreeNode{3}, new TreeNode{2}};
    assert(!leafSimilar(root1, root2));
  }
  return 0;
}