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
  int averageOfSubtree(TreeNode *root) {
    int ans = 0;
    auto dfs = [&](auto &&dfs, TreeNode *node) -> pair<int, int> {
      if (!node) {
        return {0, 0};
      }
      auto [leftSum, leftSize] = dfs(dfs, node->left);
      auto [rightSum, rightSize] = dfs(dfs, node->right);
      int Size = leftSize + rightSize + 1;
      int Sum = leftSum + rightSum + node->val;
      if (Size && Sum / Size == node->val) {
        ans++;
      }
      return {Sum, Size};
    };
    dfs(dfs, root);
    return ans;
  }
};

int main() {
  vector<tuple<TreeNode *, int>> tests{
      {
          new TreeNode(4, new TreeNode(8, new TreeNode(0), new TreeNode(1)), new TreeNode(5, nullptr, new TreeNode(6))),
          5,
      },
      {
          new TreeNode(1),
          1,
      },
  };

  for (auto &[root, ans] : tests) {
    assert(Solution().averageOfSubtree(root) == ans);
  }
}