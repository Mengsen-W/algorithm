/*
 * @Date: 2022-06-18 10:04:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-18 10:26:16
 * @FilePath: /algorithm/029_insert_into_a_sorted_circular_linked_list/insert_into_a_sorted_circular_linked_list.cpp
 */

// Definition for a Node.
class Node {
 public:
  int val;
  Node* next;

  Node() {}

  Node(int _val) {
    val = _val;
    next = nullptr;
  }

  Node(int _val, Node* _next) {
    val = _val;
    next = _next;
  }
};

class Solution {
 public:
  Node* insert(Node* head, int val) {
    if (!head) {
      Node* newNode = new Node(val);
      newNode->next = newNode;
      return newNode;
    }

    Node* ptr = head;
    while (true) {
      if (val >= ptr->val && val <= ptr->next->val) break;
      if (ptr->val > ptr->next->val && (val <= ptr->next->val || val >= ptr->val)) break;
      if (ptr->next == head) break;
      ptr = ptr->next;
    }
    ptr->next = new Node(val, ptr->next);
    return head;
  }
};

#include <cassert>
int main() {
  {
    Node* head = new Node(3, new Node(4, new Node(1)));
    assert(Solution().insert(head, 5)->val == 3);
  }

  {
    Node* head = new Node(1, new Node(0));
    assert(Solution().insert(head, 0));
  }

  {
    Node* head = nullptr;
    assert(Solution().insert(head, 1)->val == 1);
  }
}
