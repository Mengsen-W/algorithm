/*
 * @Date: 2021-09-22 09:15:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-22 09:31:53
 */

#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
 public:
  vector<ListNode *> splitListToParts(ListNode *head, int k) {
    int n = 0;
    ListNode *temp = head;
    while (temp != nullptr) {
      n++;
      temp = temp->next;
    }
    int quotient = n / k, remainder = n % k;

    vector<ListNode *> parts(k, nullptr);
    ListNode *curr = head;
    for (int i = 0; i < k && curr != nullptr; i++) {
      parts[i] = curr;
      int partSize = quotient + (i < remainder ? 1 : 0);
      for (int j = 1; j < partSize; j++) {
        curr = curr->next;
      }
      ListNode *next = curr->next;
      curr->next = nullptr;
      curr = next;
    }
    return parts;
  }
};

int main() {
  {
    ListNode *root = new ListNode{1, new ListNode{2, new ListNode{3, nullptr}}};
    int k = 5;
    vector<ListNode *> ans = Solution().splitListToParts(root, k);
    for (ListNode *head : ans)
      if (head != nullptr)
        cout << head->val << endl;
      else
        cout << "[ ]" << endl;
  }
  {
    ListNode *root = new ListNode{
        1,
        new ListNode{
            2,
            new ListNode{
                3,
                new ListNode{
                    4, new ListNode{
                           5, new ListNode{
                                  6, new ListNode{
                                         7, new ListNode{
                                                8, new ListNode{
                                                       9, new ListNode{
                                                              10,
                                                              nullptr}}}}}}}}}};
    int k = 3;
    vector<ListNode *> ans = Solution().splitListToParts(root, k);
    for (auto head : ans) {
      ListNode *cur = head;
      while (cur != nullptr) {
        cout << cur->val << endl;
        cur = cur->next;
      }
      cout << "[ ]" << endl;
    }
  }
}