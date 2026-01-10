#include <cassert>
#include <tuple>
#include <utility>
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
  pair<TreeNode *, int> f(TreeNode *root) {
    if (!root) {
      return {root, 0};
    }

    auto left = f(root->left);
    auto right = f(root->right);

    if (left.second > right.second) {
      return {left.first, left.second + 1};
    }
    if (left.second < right.second) {
      return {right.first, right.second + 1};
    }
    return {root, left.second + 1};
  }

  TreeNode *subtreeWithAllDeepest(TreeNode *root) { return f(root).first; }
};

bool is_equal(TreeNode *a, TreeNode *b) {
  if (a == nullptr && b == nullptr) {
    return true;
  }
  return a->val == b->val && is_equal(a->left, b->left) && is_equal(a->right, b->right);
}

int main() {
  vector<tuple<TreeNode *, TreeNode *>> tests{
      {
          new TreeNode{
              3,
              new TreeNode{5, new TreeNode{6}, new TreeNode{2, new TreeNode{7}, new TreeNode{4}}},
              new TreeNode{1, new TreeNode{0}, new TreeNode{8}},
          },
          new TreeNode{2, new TreeNode{7}, new TreeNode{4}},
      },
      {
          new TreeNode{1},
          new TreeNode{1},
      },
      {
          new TreeNode{
              0,
              new TreeNode{1, nullptr, new TreeNode{2}},
              new TreeNode{3},

          },
          new TreeNode{2},
      },
  };

  for (auto &[root, ans] : tests) {
    assert(is_equal(Solution().subtreeWithAllDeepest(root), ans));
  }
}