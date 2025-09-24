#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class FenwickTree {
 private:
  vector<int> tree;

 public:
  FenwickTree(int size) : tree(size + 1, 0) {}

  void update(int index, int delta) {
    index++;
    while (index < tree.size()) {
      tree[index] += delta;
      index += index & -index;
    }
  }

  int query(int index) {
    index++;
    int res = 0;
    while (index > 0) {
      res += tree[index];
      index -= index & -index;
    }
    return res;
  }
};

class Solution {
 public:
  long long goodTriplets(vector<int>& nums1, vector<int>& nums2) {
    int n = nums1.size();
    vector<int> pos2(n), reversedIndexMapping(n);
    for (int i = 0; i < n; i++) {
      pos2[nums2[i]] = i;
    }
    for (int i = 0; i < n; i++) {
      reversedIndexMapping[pos2[nums1[i]]] = i;
    }
    FenwickTree tree(n);
    long long res = 0;
    for (int value = 0; value < n; value++) {
      int pos = reversedIndexMapping[value];
      int left = tree.query(pos);
      tree.update(pos, 1);
      int right = (n - 1 - pos) - (value - left);
      res += (long long)left * right;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, long long>> tests{
      {{2, 0, 1, 3}, {0, 1, 2, 3}, 1},
      {{4, 0, 1, 3, 2}, {4, 1, 0, 2, 3}, 4},
  };

  for (auto &[nums1, nums2, ans] : tests) {
    assert(Solution().goodTriplets(nums1, nums2) == ans);
  }
}