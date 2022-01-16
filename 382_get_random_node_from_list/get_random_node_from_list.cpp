/*
 * @Date: 2022-01-16 02:41:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-16 02:47:50
 */

#include <cstdlib>
#include <vector>

using namespace std;

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution1 {
  vector<int> arr;

 public:
  Solution1(ListNode *head) {
    while (head) {
      arr.emplace_back(head->val);
      head = head->next;
    }
  }

  int getRandom() { return arr[rand() % arr.size()]; }
};

class Solution2 {
  ListNode *head;

 public:
  Solution2(ListNode *head) { this->head = head; }

  int getRandom() {
    int i = 1, ans = 0;
    for (auto node = head; node; node = node->next) {
      if (rand() % i == 0) {  // 1/i 的概率选中（替换为答案）
        ans = node->val;
      }
      ++i;
    }
    return ans;
  }
};

int main() { return 0; }