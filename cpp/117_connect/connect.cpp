/*
 * @Date: 2023-11-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-04
 * @FilePath: /algorithm/cpp/117_connect/connect.cpp
 */

// Definition for a Node.
#include <cassert>
class Node {
 public:
  int val;
  Node* left;
  Node* right;
  Node* next;
  Node() : val(0), left(nullptr), right(nullptr), next(nullptr) {}
  Node(int _val) : val(_val), left(nullptr), right(nullptr), next(nullptr) {}
  Node(int _val, Node* _left, Node* _right, Node* _next) : val(_val), left(_left), right(_right), next(_next) {}
};

#include <queue>

using namespace std;

class Solution {
 public:
  Node* connect(Node* root) {
    if (!root) {
      return nullptr;
    }
    queue<Node*> q;
    q.push(root);
    while (!q.empty()) {
      int n = q.size();
      Node* last = nullptr;
      for (int i = 1; i <= n; ++i) {
        Node* f = q.front();
        q.pop();
        if (f->left) {
          q.push(f->left);
        }
        if (f->right) {
          q.push(f->right);
        }
        if (i != 1) {
          last->next = f;
        }
        last = f;
      }
    }
    return root;
  }
};

int main() {
  Node* root =
      new Node(1, new Node(2, new Node(4), new Node(5), nullptr), new Node(3, nullptr, new Node(7), nullptr), nullptr);
  Node* ans = Solution().connect(root);
  assert(ans->left->next->val == 3);
  assert(ans->left->left->next->val == 5);
}