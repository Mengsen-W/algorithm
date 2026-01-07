
#include <cassert>
#include <cstdlib>
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
 private:
  int sum = 0;
  int best = 0;

 public:
  void dfs(TreeNode *node) {
    if (!node) {
      return;
    }
    sum += node->val;
    dfs(node->left);
    dfs(node->right);
  }

  int dfs2(TreeNode *node) {
    if (!node) {
      return 0;
    }
    int cur = dfs2(node->left) + dfs2(node->right) + node->val;
    if (abs(cur * 2 - sum) < abs(best * 2 - sum)) {
      best = cur;
    }
    return cur;
  }

  int maxProduct(TreeNode *root) {
    dfs(root);
    dfs2(root);
    return (long long)best * (sum - best) % 1000000007;
  }
};

int main() {
  std::vector<std::tuple<TreeNode *, int>> tests{
      {
          new TreeNode(1, new TreeNode(2, new TreeNode(4), new TreeNode(5)), new TreeNode(3, new TreeNode(6), nullptr)),
          110,
      },
      {
          new TreeNode(1, nullptr, new TreeNode(2, new TreeNode(3), new TreeNode(4, new TreeNode(5), new TreeNode(6)))),
          90,
      },
      {
          new TreeNode(2,
                       new TreeNode(3, new TreeNode(10, new TreeNode(5), new TreeNode(4)),
                                    new TreeNode(7, new TreeNode(11), new TreeNode(1))),
                       new TreeNode(9, new TreeNode(8), new TreeNode(6))),
          1025,
      },
      {
          new TreeNode(1, new TreeNode(1), nullptr),
          1,
      },
  };

  for (auto [root, ans] : tests) {
    assert(Solution().maxProduct(root) == ans);
  }
}