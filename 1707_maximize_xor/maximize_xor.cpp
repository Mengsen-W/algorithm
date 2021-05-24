/*
 * @Date: 2021-05-24 09:54:47
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-24 09:58:23
 */

#include <cassert>
#include <climits>
#include <vector>
using namespace std;

class Trie {
 public:
  const int L = 30;

  Trie* children[2] = {};
  int min = INT_MAX;

  void insert(int val) {
    Trie* node = this;
    node->min = std::min(node->min, val);
    for (int i = L - 1; i >= 0; --i) {
      int bit = (val >> i) & 1;
      if (node->children[bit] == nullptr) {
        node->children[bit] = new Trie();
      }
      node = node->children[bit];
      node->min = std::min(node->min, val);
    }
  }

  int getMaxXorWithLimit(int val, int limit) {
    Trie* node = this;
    if (node->min > limit) {
      return -1;
    }
    int ans = 0;
    for (int i = L - 1; i >= 0; --i) {
      int bit = (val >> i) & 1;
      if (node->children[bit ^ 1] != nullptr &&
          node->children[bit ^ 1]->min <= limit) {
        ans |= 1 << i;
        bit ^= 1;
      }
      node = node->children[bit];
    }
    return ans;
  }
};

class Solution {
 public:
  vector<int> maximizeXor(vector<int>& nums, vector<vector<int>>& queries) {
    Trie* t = new Trie();
    for (int val : nums) {
      t->insert(val);
    }
    int numQ = queries.size();
    vector<int> ans(numQ);
    for (int i = 0; i < numQ; ++i) {
      ans[i] = t->getMaxXorWithLimit(queries[i][0], queries[i][1]);
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{0, 1, 2, 3, 4};
    vector<vector<int>> queries{{3, 1}, {1, 3}, {5, 6}};
    assert(Solution{}.maximizeXor(nums, queries) ==
           std::move(vector<int>{3, 3, 7}));
  }
  {
    vector<int> nums{5, 2, 4, 6, 6, 3};
    vector<vector<int>> queries{{12, 4}, {8, 1}, {6, 3}};
    assert(Solution{}.maximizeXor(nums, queries) ==
           std::move(vector<int>{15, -1, 5}));
  }
}