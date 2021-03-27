/*
 * @Date: 2021-03-25 09:05:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-26 09:04:28
 */

#include <iostream>

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

ListNode* delete_duplicates(ListNode* head) {
  if (!head) return head;
  ListNode* dummy = new ListNode(0, head);
  ListNode* cur = dummy;

  while (cur->next && cur->next->next) {
    if (cur->next->val == cur->next->next->val) {
      int temp = cur->next->val;
      while (cur->next && cur->next->val == temp) {
        cur->next = cur->next->next;
      }
    } else
      cur = cur->next;
  }
  return dummy->next;
}

int main() {
  ListNode* head = new ListNode(
      1,
      new ListNode(
          2,
          new ListNode(
              3, new ListNode(
                     3, new ListNode(4, new ListNode(4, new ListNode(5)))))));
  ListNode* cur = delete_duplicates(head);
  while (cur) {
    std::cout << cur->val << ", ";
    cur = cur->next;
  }
  std::cout << std::endl;

  head = new ListNode(
      1, new ListNode(1, new ListNode(1, new ListNode(2, new ListNode(3)))));
  cur = delete_duplicates(head);
  while (cur) {
    std::cout << cur->val << ", ";
    cur = cur->next;
  }
  return 0;
}