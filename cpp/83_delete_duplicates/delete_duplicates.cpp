/*
 * @Date: 2021-03-26 08:49:15
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-14
 */

#include <cassert>
#include <tuple>
#include <vector>

struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr){};
  ListNode(int x) : val(x), next(nullptr){};
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode *deleteDuplicates(ListNode *head) {
    if (head == nullptr) return head;
    ListNode *dummy = new ListNode(0, head);
    ListNode *cur = dummy;
    while (cur->next && cur->next->next) {
      if (cur->next->val == cur->next->next->val) {
        int temp = cur->next->val;
        while (cur->next->next && cur->next->next->val == temp) {
          cur->next = cur->next->next;
        }
      } else
        cur = cur->next;
    }
    return dummy->next;
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
      {new ListNode(1, new ListNode(1, new ListNode(2))), new ListNode(1, new ListNode(2))},
      {new ListNode(1, new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(3))))),
       new ListNode(1, new ListNode(2, new ListNode(3)))},
  };

  for (auto &[l, ans] : tests) {
    assert(is_same_list(Solution().deleteDuplicates(l), ans) == true);
  }

  return 0;
}