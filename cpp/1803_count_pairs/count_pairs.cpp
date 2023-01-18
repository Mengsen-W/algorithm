/*
 * @Date: 2023-01-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-05
 * @FilePath: /algorithm/1803_count_pairs/count_pairs.cpp
 */

#include <array>
#include <cassert>
#include <vector>

using namespace std;

struct Trie {
  // son[0] 表示左子树，son[1] 表示右子树
  array<Trie*, 2> son{nullptr, nullptr};
  int sum;
  Trie() : sum(0) {}
};

class Solution {
 private:
  // 字典树的根节点
  Trie* root = nullptr;
  // 最高位的二进制位编号为 14
  static constexpr int HIGH_BIT = 14;

 public:
  void add(int num) {
    Trie* cur = root;
    for (int k = HIGH_BIT; k >= 0; k--) {
      int bit = (num >> k) & 1;
      if (cur->son[bit] == nullptr) {
        cur->son[bit] = new Trie();
      }
      cur = cur->son[bit];
      cur->sum++;
    }
  }

  int get(int num, int x) {
    Trie* cur = root;
    int sum = 0;
    for (int k = HIGH_BIT; k >= 0; k--) {
      int r = (num >> k) & 1;
      if ((x >> k) & 1) {
        if (cur->son[r] != nullptr) {
          sum += cur->son[r]->sum;
        }
        if (cur->son[r ^ 1] == nullptr) {
          return sum;
        }
        cur = cur->son[r ^ 1];
      } else {
        if (cur->son[r] == nullptr) {
          return sum;
        }
        cur = cur->son[r];
      }
    }
    sum += cur->sum;
    return sum;
  }

  int f(vector<int>& nums, int x) {
    root = new Trie();
    int res = 0;
    for (int i = 1; i < nums.size(); i++) {
      add(nums[i - 1]);
      res += get(nums[i], x);
    }
    return res;
  }

  int countPairs(vector<int>& nums, int low, int high) { return f(nums, high) - f(nums, low - 1); }
};

int main() {
  {
    vector<int> nums{1, 4, 2, 7};
    int low = 2;
    int high = 6;
    int ans = 6;
    assert(Solution().countPairs(nums, low, high) == ans);
  }

  {
    vector<int> nums{9,8,4,2,1};
    int low = 5;
    int high = 14;
    int ans = 8;
    assert(Solution().countPairs(nums, low, high) == ans);
  }
}