/*
 * @Date: 2021-09-02 13:02:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-02 13:12:20
 */

#include <cassert>

struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* getKthFromEnd(ListNode* head, int k) {
    ListNode* fast = head;
    ListNode* slow = head;

    while (fast && k > 0) {
      fast = fast->next;
      k--;
    }
    while (fast) {
      fast = fast->next;
      slow = slow->next;
    }

    return slow;
  }
};

int main() {
  ListNode* root = new ListNode{
      1, new ListNode{
             2, new ListNode{3, new ListNode{4, new ListNode{5, nullptr}}}}};
  int k = 2;
  ListNode* res = Solution().getKthFromEnd(root, k);
  assert(res->val == 4);
  assert(res->next->val == 5);
}