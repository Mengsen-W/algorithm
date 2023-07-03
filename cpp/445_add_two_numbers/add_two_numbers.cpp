/*
 * @Date: 2023-07-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-03
 * @FilePath: /algorithm/cpp/445_add_two_numbers/add_two_numbers.cpp
 */

#include <cassert>
#include <cstddef>
#include <stack>
#include <tuple>
#include <vector>

using namespace std;

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
  ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
    stack<int> s1, s2;
    while (l1) {
      s1.push(l1->val);
      l1 = l1->next;
    }
    while (l2) {
      s2.push(l2->val);
      l2 = l2->next;
    }
    int carry = 0;
    ListNode* ans = nullptr;
    while (!s1.empty() or !s2.empty() or carry != 0) {
      int a = s1.empty() ? 0 : s1.top();
      int b = s2.empty() ? 0 : s2.top();
      if (!s1.empty()) s1.pop();
      if (!s2.empty()) s2.pop();
      int cur = a + b + carry;
      carry = cur / 10;
      cur %= 10;
      auto curnode = new ListNode(cur);
      curnode->next = ans;
      ans = curnode;
    }
    return ans;
  }
};

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

bool same(ListNode* l1, ListNode* l2) {
  if (l1 == nullptr && l2 == nullptr) {
    return true;
  }

  if (l1 != nullptr && l2 != nullptr) {
    return l1->val == l2->val && same(l1->next, l2->next);
  }
  return false;
}

int main() {
  vector<tuple<vector<int>, vector<int>, vector<int>>> test_cases{
      {vector<int>{7, 2, 4, 3}, vector<int>{5, 6, 4}, vector<int>{7, 8, 0, 7}},
      {vector<int>{2, 4, 3}, vector<int>{5, 6, 4}, vector<int>{8, 0, 7}},
      {vector<int>{0}, vector<int>{0}, vector<int>{0}},
  };

  for (auto& [l1, l2, ans] : test_cases) {
    assert(same(Solution().addTwoNumbers(vec_to_list(l1), vec_to_list(l2)), vec_to_list(ans)));
  }
}