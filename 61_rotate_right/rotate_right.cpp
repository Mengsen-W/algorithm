/*
 * @Date: 2021-03-27 07:54:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-27 08:21:23
 */

#include <iostream>

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

ListNode* rotate_right(ListNode* head, int k) {
  if (head == nullptr || head->next == nullptr) return head;
  ListNode* last = head;
  int count = 1;

  // 计算链表总长
  while (last->next != nullptr) {
    last = last->next;
    ++count;
  }

  ListNode* front = head;
  // 必要的优化
  if (k > count) k %= count;

  // 链表头拼接到末尾
  while (count - k) {
    last->next = front;
    front = front->next;
    last = last->next;
    ++k;
  }
  last->next = nullptr;
  return front;
}

int main() {
  {
    ListNode* head_1 = new ListNode(
        1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(5)))));
    ListNode* cur = rotate_right(head_1, 2);
    while (cur != nullptr) {
      std::cout << cur->val << ", ";
      cur = cur->next;
    }
  }
  std::cout << std::endl;

  {
    ListNode* head_2 = new ListNode(0, new ListNode(1, new ListNode(2)));
    ListNode* cur = rotate_right(head_2, 4);
    while (cur != nullptr) {
      std::cout << cur->val << ", ";
      cur = cur->next;
    }
  }

  return 0;
}