/*
 * @Date: 2021-07-22 22:02:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-22 22:03:25
 */

class Node {
 public:
  int val;
  Node* next;
  Node* random;

  Node(int _val) {
    val = _val;
    next = nullptr;
    random = nullptr;
  }
};

class Solution {
 public:
  Node* copyRandomList(Node* head) {
    if (head == nullptr) {
      return nullptr;
    }
    for (Node* node = head; node != nullptr; node = node->next->next) {
      Node* nodeNew = new Node(node->val);
      nodeNew->next = node->next;
      node->next = nodeNew;
    }
    for (Node* node = head; node != nullptr; node = node->next->next) {
      Node* nodeNew = node->next;
      nodeNew->random =
          (node->random != nullptr) ? node->random->next : nullptr;
    }
    Node* headNew = head->next;
    for (Node* node = head; node != nullptr; node = node->next) {
      Node* nodeNew = node->next;
      node->next = node->next->next;
      nodeNew->next =
          (nodeNew->next != nullptr) ? nodeNew->next->next : nullptr;
    }
    return headNew;
  }
};