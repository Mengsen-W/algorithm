/*
 * @Date: 2021-06-05 09:12:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-05 10:43:38
 */

#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

ListNode* removeElements_recursive(ListNode* head, int val) {
  if (head == nullptr) {
    return head;
  }
  head->next = removeElements_recursive(head->next, val);
  return head->val == val ? head->next : head;
}

ListNode* removeElements_iteration(ListNode* head, int val) {
  struct ListNode* dummyHead = new ListNode(0, head);
  struct ListNode* temp = dummyHead;
  while (temp->next != nullptr) {
    if (temp->next->val == val) {
      temp->next = temp->next->next;
    } else {
      temp = temp->next;
    }
  }
  return dummyHead->next;
}

ListNode* vec_to_list(const vector<int>& nums) {
  int size = nums.size();
  if (size == 0) return nullptr;
  ListNode* res = new ListNode();
  ListNode* cur = res;
  for (int i = 0; i < size; ++i) {
    cur->val = nums[i];
    if (i == size - 1) {
      cur->next = nullptr;
    } else {
      cur->next = new ListNode();
    }
    cur = cur->next;
  }
  return res;
}

void print_list(ListNode* list) {
  while (list != nullptr) {
    cout << list->val;
    list = list->next;
  }
  cout << endl;
}

int main() {
  print_list(removeElements_recursive(
      vec_to_list(vector<int>{1, 2, 6, 3, 4, 5, 6}), 6));
  print_list(removeElements_recursive(vec_to_list(vector<int>{}), 1));
  print_list(removeElements_recursive(vec_to_list(vector<int>{7, 7, 7, 7}), 7));
  print_list(removeElements_iteration(
      vec_to_list(vector<int>{1, 2, 6, 3, 4, 5, 6}), 6));
  print_list(removeElements_iteration(vec_to_list(vector<int>{}), 1));
  print_list(removeElements_iteration(vec_to_list(vector<int>{7, 7, 7, 7}), 7));
  return 0;
}
