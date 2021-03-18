/*
 * @Date: 2021-03-18 08:44:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-18 09:13:09
 */

#include <iostream>

struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
 public:
  ListNode* successor = nullptr;  // 后驱节点

  ListNode* reverse_between(ListNode* head, int m, int n) {
    if (m == 1) {
      return reverse_n(head, n);
    }

    head->next = reverse_between(head->next, m - 1, n - 1);
    return head;
  }

  ListNode* reverse_n(ListNode* head, int n) {
    if (n == 1) {
      successor = head->next;
      return head;
    }

    ListNode* last = reverse_n(head->next, n - 1);
    // 局部反转
    head->next->next = head;
    // 链接到下一节点
    head->next = successor;
    // last 为结果头结点
    return last;
  }
};

int main() {
  ListNode* n_1 = new ListNode(1);
  ListNode* n_2 = new ListNode(2);
  ListNode* n_3 = new ListNode(3);
  ListNode* n_4 = new ListNode(4);
  ListNode* n_5 = new ListNode(5);

  n_1->next = n_2;
  n_2->next = n_3;
  n_3->next = n_4;
  n_4->next = n_5;
  n_5->next = nullptr;

  Solution solution{};

  ListNode* res = solution.reverse_between(n_1, 2, 4);

  while (n_1->next != nullptr) {
    std::cout << res->val << std::endl;
    res = res->next;
  }

  return 0;
}