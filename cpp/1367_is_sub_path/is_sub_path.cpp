
#include <cassert>
#include <tuple>
#include <vector>

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

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
  bool dfs(TreeNode *rt, ListNode *head) {
    // 链表已经全部匹配完，匹配成功
    if (head == nullptr) return true;
    // 二叉树访问到了空节点，匹配失败
    if (rt == nullptr) return false;
    // 当前匹配的二叉树上节点的值与链表节点的值不相等，匹配失败
    if (rt->val != head->val) return false;
    return dfs(rt->left, head->next) || dfs(rt->right, head->next);
  }

 public:
  bool isSubPath(ListNode *head, TreeNode *root) {
    if (root == nullptr) return false;
    return dfs(root, head) || isSubPath(head, root->left) || isSubPath(head, root->right);
  }
};

int main() {
  std::vector<std::tuple<ListNode *, TreeNode *, bool>> tests{
      {
          new ListNode(4, new ListNode(2, new ListNode(8))),
          new TreeNode(
              1, new TreeNode(4, nullptr, new TreeNode(2, new TreeNode(1, nullptr, nullptr), nullptr)),
              new TreeNode(4, new TreeNode(2, new TreeNode(6), new TreeNode(8, new TreeNode(1), new TreeNode(3))),
                           nullptr)),
          true,
      },
      {
          new ListNode(1, new ListNode(4, new ListNode(2, new ListNode(6)))),
          new TreeNode(
              1, new TreeNode(4, nullptr, new TreeNode(2, new TreeNode(1, nullptr, nullptr), nullptr)),
              new TreeNode(4, new TreeNode(2, new TreeNode(6), new TreeNode(8, new TreeNode(1), new TreeNode(3))),
                           nullptr)),
          true,
      },
      {
          new ListNode(1, new ListNode(4, new ListNode(2, new ListNode(6, new ListNode(8))))),
          new TreeNode(
              1, new TreeNode(4, nullptr, new TreeNode(2, new TreeNode(1, nullptr, nullptr), nullptr)),
              new TreeNode(4, new TreeNode(2, new TreeNode(6), new TreeNode(8, new TreeNode(1), new TreeNode(3))),
                           nullptr)),
          false,
      },
  };

  for (auto &[head, root, ans] : tests) {
      assert(Solution().isSubPath(head, root) == ans);
  }
}