/*
 * @Date: 2023-08-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-14
 * @FilePath: /algorithm/cpp/617_merge_trees/merge_trees.cpp
 */

#include <cassert>
#include <functional>
#include <tuple>
#include <vector>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <queue>

using namespace std;

class Solution {
 public:
  TreeNode *mergeTrees(TreeNode *t1, TreeNode *t2) {
    if (t1 == nullptr) {
      return t2;
    }
    if (t2 == nullptr) {
      return t1;
    }
    auto merged = new TreeNode(t1->val + t2->val);
    merged->left = mergeTrees(t1->left, t2->left);
    merged->right = mergeTrees(t1->right, t2->right);
    return merged;
  }
};

vector<int> preorderTraversal(TreeNode *root) {
  vector<int> res;
  function<void(TreeNode *, vector<int> &)> preorder = [&preorder](TreeNode *root, vector<int> &res) -> void {
    if (root == nullptr) {
      return;
    }
    res.push_back(root->val);
    preorder(root->left, res);
    preorder(root->right, res);
  };
  preorder(root, res);
  return res;
}

int main() {
  vector<tuple<TreeNode *, TreeNode *, TreeNode *>> test_cases = {
      {
          new TreeNode(1, new TreeNode(3, new TreeNode(5), nullptr), new TreeNode(2)),
          new TreeNode(2, new TreeNode(1, nullptr, new TreeNode(4)), new TreeNode(3, nullptr, new TreeNode(7))),
          new TreeNode(3, new TreeNode(4, new TreeNode(5), new TreeNode(4)), new TreeNode(5, nullptr, new TreeNode(7))),
      },
      {
          new TreeNode(1),
          new TreeNode(1, new TreeNode(2), nullptr),
          new TreeNode(2, new TreeNode(2), nullptr),
      },
  };

  for (auto &[t1, t2, expected] : test_cases) {
    assert(preorderTraversal(Solution().mergeTrees(t1, t2)) == preorderTraversal(expected));
  }
}