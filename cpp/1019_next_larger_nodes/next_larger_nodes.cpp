/*
 * @Date: 2023-04-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-10
 * @FilePath: /algorithm/cpp/1019_next_larger_nodes/next_larger_nodes.cpp
 */

#include <cassert>
#include <stack>
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
  vector<int> nextLargerNodes(ListNode* head) {
    vector<int> ans;
    stack<pair<int, int>> s;

    ListNode* cur = head;
    int idx = -1;
    while (cur) {
      ++idx;
      ans.push_back(0);
      while (!s.empty() && s.top().first < cur->val) {
        ans[s.top().second] = cur->val;
        s.pop();
      }
      s.emplace(cur->val, idx);
      cur = cur->next;
    }

    return ans;
  }
};

int main() {
  {
    ListNode* head = new ListNode{2, new ListNode{1, new ListNode{5}}};
    vector<int> ans{5, 5, 0};
    assert(Solution().nextLargerNodes(head) == ans);
  }

  {
    ListNode* head = new ListNode{2, new ListNode{7, new ListNode{4, new ListNode{3, new ListNode{5}}}}};
    vector<int> ans{7, 0, 5, 5, 0};
    assert(Solution().nextLargerNodes(head) == ans);
  }
}