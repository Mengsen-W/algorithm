/*
 * @Date: 2024-01-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-03
 * @FilePath: /algorithm/cpp/2487_remove_nodes/remove_nodes.cpp
 */

// Definition for singly-linked list.
#include <cassert>
#include <tuple>
#include <vector>

struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode *reverse(ListNode *head) {
    ListNode dummy;
    while (head != nullptr) {
      ListNode *p = head;
      head = head->next;
      p->next = dummy.next;
      dummy.next = p;
    }
    return dummy.next;
  }

  ListNode *removeNodes(ListNode *head) {
    head = reverse(head);
    for (ListNode *p = head; p->next != nullptr;) {
      if (p->val > p->next->val) {
        p->next = p->next->next;
      } else {
        p = p->next;
      }
    }
    return reverse(head);
  }
};

bool is_same_list(ListNode *l1, ListNode *l2) {
  while (l1 != NULL && l2 != NULL) {
    if (l1->val != l2->val) {
      return false;
    }
    l1 = l1->next;
    l2 = l2->next;
  }

  // 如果其中一个链表遍历完了，但另一个链表还没有遍历完，那么它们不相等。
  if (l1 != NULL || l2 != NULL) {
    return false;
  }

  return true;
}

int main() {
  std::vector<std::tuple<ListNode *, ListNode *>> tests{
      {
          new ListNode{5, new ListNode{2, new ListNode{13, new ListNode{3, new ListNode{8}}}}},
          new ListNode{13, new ListNode{8}},
      },
      {
          new ListNode{1, new ListNode{1, new ListNode{1, new ListNode{1}}}},
          new ListNode{1, new ListNode{1, new ListNode{1, new ListNode{1}}}},
      },
  };

  for (auto &[l1, ans] : tests) {
    assert(is_same_list(Solution().removeNodes(l1), ans));
  }
}