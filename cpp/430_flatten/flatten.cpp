/*
 * @Date: 2021-09-24 09:55:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-24 10:06:56
 */

#include <functional>
using namespace std;

// Definition for a Node.
class Node {
 public:
  int val;
  Node* prev;
  Node* next;
  Node* child;
};

class Solution {
 public:
  Node* flatten(Node* head) {
    function<Node*(Node*)> dfs = [&](Node* node) {
      Node* cur = node;
      // 记录链表的最后一个节点
      Node* last = nullptr;

      while (cur) {
        Node* next = cur->next;
        //  如果有子节点，那么首先处理子节点
        if (cur->child) {
          Node* child_last = dfs(cur->child);

          next = cur->next;
          //  将 node 与 child 相连
          cur->next = cur->child;
          cur->child->prev = cur;

          //  如果 next 不为空，就将 last 与 next 相连
          if (next) {
            child_last->next = next;
            next->prev = child_last;
          }

          // 将 child 置为空
          cur->child = nullptr;
          last = child_last;
        } else {
          last = cur;
        }
        cur = next;
      }
      return last;
    };

    dfs(head);
    return head;
  }
};

int main() {}
