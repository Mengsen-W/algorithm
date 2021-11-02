/*
 * @Date: 2021-11-02 01:11:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-02 01:28:46
 */

#include <cassert>

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
 public:
  // 想要删除的节点的值替换为它后面节点中的值，然后删除它之后的节点
  void deleteNode(ListNode* node) {
    ListNode* next = node->next;
    *node = *(next);
    delete next;
  }
};

int main() {
  ListNode* head = new ListNode(4);
  head->next = new ListNode(5);
  head->next->next = new ListNode(1);
  head->next->next->next = new ListNode(9);
  Solution solution;
  solution.deleteNode(head->next);
  assert(head->next->next->next == nullptr);
  assert(head->next->next->val == 9);
  return 0;
}