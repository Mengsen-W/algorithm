/*
 * @Date: 2023-05-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-30
 * @FilePath: /algorithm/cpp/1110_del_nodes/del_nodes.cpp
 */

#include <cassert>
#include <cstddef>
#include <unordered_set>
#include <vector>

using namespace std;

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

class Solution {
 public:
  vector<TreeNode*> delNodes(TreeNode* root, vector<int>& to_delete) {
    unordered_set<int> to_delete_set(to_delete.begin(), to_delete.end());
    vector<TreeNode*> roots;

    function<TreeNode*(TreeNode*, bool)> dfs = [&](TreeNode* node, bool is_root) -> TreeNode* {
      if (node == nullptr) {
        return nullptr;
      }
      bool deleted = to_delete_set.count(node->val) ? true : false;
      node->left = dfs(node->left, deleted);
      node->right = dfs(node->right, deleted);
      if (deleted) {
        return nullptr;
      } else {
        if (is_root) {
          roots.emplace_back(node);
        }
        return node;
      }
    };

    dfs(root, true);
    return roots;
  }
};

int main() {
  {
    TreeNode* root = new TreeNode(1, new TreeNode(2, new TreeNode(4), new TreeNode(5)),
                                  new TreeNode(3, new TreeNode(6), new TreeNode(7)));
    vector<int> to_delete{3, 5};
    vector<TreeNode*> ans{
        new TreeNode(6),
        new TreeNode(1, new TreeNode(2, nullptr, new TreeNode(4)), nullptr),
        new TreeNode(7),
    };
    assert(Solution().delNodes(root, to_delete) == ans);
  }

  {
    TreeNode* root = new TreeNode(1, new TreeNode(2, nullptr, new TreeNode(3)), new TreeNode(4));
    vector<int> to_delete{3};
    vector<TreeNode*> ans{new TreeNode(1, new TreeNode(2), new TreeNode(4))};
    assert(Solution().delNodes(root, to_delete) == ans);
  }
}