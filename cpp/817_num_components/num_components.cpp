/*
 * @Date: 2022-10-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-12
 * @FilePath: /algorithm/817_num_components/num_components.cpp
 */

#include <cassert>
#include <unordered_set>
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
  int numComponents(ListNode* head, vector<int>& nums) {
    unordered_set<int> numsSet;
    for (int num : nums) {
      numsSet.emplace(num);
    }
    bool inSet = false;
    int res = 0;
    while (head != nullptr) {
      if (numsSet.count(head->val)) {
        if (!inSet) {
          inSet = true;
          res++;
        }
      } else {
        inSet = false;
      }
      head = head->next;
    }
    return res;
  }
};

int main() {
  {
    ListNode* header = new ListNode{0, new ListNode{1, new ListNode{2, new ListNode{3}}}};
    vector<int> nums{0, 1, 3};
    int ans = 2;
    assert(Solution().numComponents(header, nums) == ans);
  }

  {
    ListNode* header = new ListNode{0, new ListNode{1, new ListNode{2, new ListNode{3, new ListNode{4}}}}};
    vector<int> nums{0, 3, 1, 4};
    int ans = 2;
    assert(Solution().numComponents(header, nums) == ans);
  }
}