/*
 * @Date: 2023-08-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-12
 * @FilePath: /algorithm/cpp/23_merge_k_lists/merge_k_lists.cpp
 */

// Definition for singly-linked list.
#include <cassert>
#include <tuple>
#include <vector>
struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

#include <queue>
using namespace std;

class Solution {
 public:
  struct Status {
    int val;
    ListNode* ptr;
    bool operator<(const Status& rhs) const { return val > rhs.val; }
  };

  priority_queue<Status> q;

  ListNode* mergeKLists(vector<ListNode*>& lists) {
    for (auto node : lists) {
      if (node) q.push({node->val, node});
    }
    ListNode head, *tail = &head;
    while (!q.empty()) {
      auto f = q.top();
      q.pop();
      tail->next = f.ptr;
      tail = tail->next;
      if (f.ptr->next) q.push({f.ptr->next->val, f.ptr->next});
    }
    return head.next;
  }
};

vector<int> list_to_vec(ListNode* l) {
  ListNode* step = l;
  vector<int> ans{};
  while (step != nullptr) {
    ans.push_back(step->val);
    step = step->next;
  }
  return ans;
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

int main() {
  vector<tuple<vector<ListNode*>, vector<int>>> test_cases{
      {{vec_to_list({1, 4, 5}), vec_to_list({1, 3, 4}), vec_to_list({2, 6})}, {1, 1, 2, 3, 4, 4, 5, 6}},
      {{}, {}},
      {{vec_to_list({})}, {}},
  };
  Solution s;
  for (auto& [lists, ans] : test_cases) {
    assert(list_to_vec(s.mergeKLists(lists)) == (ans));
  }
}
