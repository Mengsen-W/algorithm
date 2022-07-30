/*
 * @Date: 2022-07-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-30
 * @FilePath: /algorithm/952_largest_component_size/largest_component_size.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class UnionFind {
 public:
  UnionFind(int n) {
    parent = vector<int>(n);
    rank = vector<int>(n);
    for (int i = 0; i < n; i++) {
      parent[i] = i;
    }
  }

  void uni(int x, int y) {
    int rootx = find(x);
    int rooty = find(y);
    if (rootx != rooty) {
      if (rank[rootx] > rank[rooty]) {
        parent[rooty] = rootx;
      } else if (rank[rootx] < rank[rooty]) {
        parent[rootx] = rooty;
      } else {
        parent[rooty] = rootx;
        rank[rootx]++;
      }
    }
  }

  int find(int x) {
    if (parent[x] != x) {
      parent[x] = find(parent[x]);
    }
    return parent[x];
  }

 private:
  vector<int> parent;
  vector<int> rank;
};

class Solution {
 public:
  int largestComponentSize(vector<int>& nums) {
    int m = *max_element(nums.begin(), nums.end());
    UnionFind uf(m + 1);
    for (int num : nums) {
      for (int i = 2; i * i <= num; i++) {
        if (num % i == 0) {
          uf.uni(num, i);
          uf.uni(num, num / i);
        }
      }
    }
    vector<int> counts(m + 1);
    int ans = 0;
    for (int num : nums) {
      int root = uf.find(num);
      counts[root]++;
      ans = max(ans, counts[root]);
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{4, 6, 15, 35};
    int ans = 4;
    assert(Solution().largestComponentSize(nums) == ans);
  }
  {
    vector<int> nums{20, 50, 9, 63};
    int ans = 2;
    assert(Solution().largestComponentSize(nums) == ans);
  }
  {
    vector<int> nums{2, 3, 6, 7, 4, 12, 21, 39};
    int ans = 8;
    assert(Solution().largestComponentSize(nums) == ans);
  }
}
