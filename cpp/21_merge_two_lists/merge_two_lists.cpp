/*
 * @Date: 2023-08-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-05
 * @FilePath: /algorithm/cpp/21_merge_two_lists/merge_two_lists.cpp
 */

// Definition for singly-linked list.
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
    ListNode* preHead = new ListNode(-1);

    ListNode* prev = preHead;
    while (l1 != nullptr && l2 != nullptr) {
      if (l1->val < l2->val) {
        prev->next = l1;
        l1 = l1->next;
      } else {
        prev->next = l2;
        l2 = l2->next;
      }
      prev = prev->next;
    }

    // 合并后 l1 和 l2 最多只有一个还未被合并完，我们直接将链表末尾指向未合并完的链表即可
    prev->next = l1 == nullptr ? l2 : l1;

    return preHead->next;
  }
};

string listNodeToString(ListNode* header) {
  string result = "[";
  if (header) {
    result += to_string(header->val);
    result += listNodeToString(header->next);
  }
  return result + "]";
}

int main() {
  std::vector<std::tuple<ListNode*, ListNode*, ListNode*>> test_cases{
      {
          new ListNode(1, new ListNode(2, new ListNode(4))),
          new ListNode(1, new ListNode(3, new ListNode(4))),
          new ListNode(1, new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(4)))))),
      },
      {
          nullptr,
          nullptr,
          nullptr,
      },
      {
          nullptr,
          new ListNode(0),
          new ListNode(0),
      },
  };

  for (auto& [l1, l2, ans] : test_cases) {
    assert(listNodeToString(Solution().mergeTwoLists(l1, l2)) == listNodeToString(ans));
  }
}