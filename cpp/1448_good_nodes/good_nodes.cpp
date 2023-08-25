/*
 * @Date: 2023-08-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-25
 * @FilePath: /algorithm/cpp/1448_good_nodes/good_nodes.cpp
 */

//  Definition for a binary tree node.
#include <cassert>
#include <climits>
#include <cstddef>
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

class Solution {
 public:
  int goodNodes(TreeNode *root) {
    // 返回值就是个数
    std::function<int(TreeNode *, int)> dfs = [&dfs](TreeNode *root, int max_val) -> int {
      if (root == nullptr) {
        return 0;
      }
      int res = 0;
      if (root->val >= max_val) {
        res++;
        max_val = root->val;
      }
      return res + dfs(root->left, max_val) + dfs(root->right, max_val);
    };

    return dfs(root, INT_MIN);
  }
};

int main() {
  std::vector<std::tuple<TreeNode *, int>> test_cases{
      {
          new TreeNode(3, new TreeNode(1, new TreeNode(3), nullptr), new TreeNode(4, new TreeNode(1), new TreeNode(5))),
          4,
      },
      {
          new TreeNode(3, new TreeNode(3, new TreeNode(4), new TreeNode(2)), nullptr),
          3,
      },
      {
          new TreeNode(3, nullptr, nullptr),
          1,
      },
  };

  for (auto &[root, expect] : test_cases) {
    assert(Solution().goodNodes(root) == expect);
  }
}