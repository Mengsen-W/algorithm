/*
 * @Date: 2024-01-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-06
 * @FilePath: /algorithm/cpp/2807_insert_greatest_common_divisors/insert_greatest_common_divisors.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* insertGreatestCommonDivisors(ListNode* head) {
    ListNode* node = head;
    while (node->next) {
      node->next = new ListNode(std::gcd(node->val, node->next->val), node->next);
      node = node->next->next;
    }
    return head;
  }
};

bool is_same_list(ListNode* l1, ListNode* l2) {
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
  std::vector<std::tuple<ListNode*, ListNode*>> tests{
      {new ListNode{
           18,
           new ListNode{6, new ListNode{10, new ListNode{3}}},
       },
       new ListNode{
           18, new ListNode{6, new ListNode{6, new ListNode{2, new ListNode{10, new ListNode{1, new ListNode{3}}}}}}}},
      {new ListNode{7}, new ListNode{7}},
  };

  for (auto& [head, ans] : tests) {
    assert(is_same_list(Solution().insertGreatestCommonDivisors(head), ans) == true);
  }
}