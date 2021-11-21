/*
 * @Date: 2021-11-21 01:41:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-21 02:10:15
 */

#include <cassert>
#include <queue>
#include <vector>

using namespace std;

// Definition for a Node.
class Node {
 public:
  int val;
  vector<Node *> children;

  Node() {}

  Node(int _val) { val = _val; }

  Node(int _val, vector<Node *> _children) {
    val = _val;
    children = _children;
  }
};

class Solution {
 public:
  int maxDepth(Node *root) {
    if (root == nullptr) return 0;
    queue<Node *> qu;
    qu.push(root);
    int ans = 0;
    while (!qu.empty()) {
      int size = qu.size();
      while (size > 0) {
        Node *node = qu.front();
        qu.pop();
        vector<Node *> children = node->children;
        for (auto child : children) qu.push(child);
        size--;
      }
      ans++;
    }
    return ans;
  }
};

int main() {
  {
    Node *root = new Node(
        1, vector<Node *>{new Node(2),
                          new Node(3, vector<Node *>{new Node(5), new Node(6)}),
                          new Node(4)});
    assert(Solution().maxDepth(root) == 3);
  }
  {
    Node *root = new Node(
        1,
        vector<Node *>{
            new Node(2),
            new Node(3,
                     vector<Node *>{
                         new Node(6),
                         new Node(7, vector<Node *>{new Node(
                                         11, vector<Node *>{new Node(14)})})}),
            new Node(4,
                     vector<Node *>{new Node(8, vector<Node *>{new Node(12)})}),

            new Node(5,
                     vector<Node *>{new Node(9, vector<Node *>{new Node(13)}),
                                    new Node(10)})});
    assert(Solution().maxDepth(root) == 5);
  }
}