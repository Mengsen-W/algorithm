#include <cassert>
#include <tuple>
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
  int maxLevelSum(TreeNode *root) {
    int ans = 1, maxSum = root->val;
    vector<TreeNode *> q = {root};
    for (int level = 1; !q.empty(); ++level) {
      vector<TreeNode *> nq;
      int sum = 0;
      for (auto node : q) {
        sum += node->val;
        if (node->left) {
          nq.emplace_back(node->left);
        }
        if (node->right) {
          nq.emplace_back(node->right);
        }
      }
      if (sum > maxSum) {
        maxSum = sum;
        ans = level;
      }
      q = std::move(nq);
    }
    return ans;
  }
};

int main() {
  vector<tuple<TreeNode *, int>> tests{
      {
          new TreeNode(1, new TreeNode(7, new TreeNode(7), new TreeNode(-8)), new TreeNode(0)),
          2,
      },
      {
          new TreeNode(989, nullptr,
                       new TreeNode(10250, new TreeNode(98693), new TreeNode(-89388, nullptr, new TreeNode(-32127)))),
          2,
      },
  };

  for (auto [root, expected] : tests) {
    assert(Solution().maxLevelSum(root) == expected);
  }
}