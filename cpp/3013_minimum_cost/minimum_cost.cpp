#include <cassert>
#include <set>
#include <tuple>
#include <vector>
using namespace std;

// 两个 multiset 维护前 k 小值
class Container {
 public:
  Container(int k) : k(k), sm(0) {}

  // 调整有序集合的大小，保证调整后前 k 个最小值均在 st1
  void adjust() {
    while (st1.size() < k && st2.size() > 0) {
      int x = *(st2.begin());
      st1.emplace(x);
      sm += x;
      st2.erase(st2.begin());
    }
    while (st1.size() > k) {
      int x = *prev(st1.end());
      st2.emplace(x);
      st1.erase(prev(st1.end()));
      sm -= x;
    }
  }

  // 插入元素 x
  void add(int x) {
    if (!st2.empty() && x >= *(st2.begin())) {
      st2.emplace(x);
    } else {
      st1.emplace(x);
      sm += x;
    }
    adjust();
  }

  // 删除元素 x
  void erase(int x) {
    auto it = st1.find(x);
    if (it != st1.end()) {
      st1.erase(it), sm -= x;
    } else {
      st2.erase(st2.find(x));
    }
    adjust();
  }

  // 前 k 小元素的和
  long long sum() { return sm; }

 private:
  int k;
  // st1 保存前 k 小值，st2 保存其它值
  multiset<int> st1, st2;
  // sm 表示前 k 小元素的和
  long long sm;
};

class Solution {
 public:
  long long minimumCost(vector<int>& nums, int k, int dist) {
    int n = nums.size();
    // 滑动窗口初始化
    Container cnt(k - 2);
    for (int i = 1; i < k - 1; i++) {
      cnt.add(nums[i]);
    }

    long long ans = cnt.sum() + nums[k - 1];
    // 枚举最后一个数组的开头
    for (int i = k; i < n; i++) {
      int j = i - dist - 1;
      if (j > 0) {
        cnt.erase(nums[j]);
      }
      cnt.add(nums[i - 1]);
      ans = min(ans, cnt.sum() + nums[i]);
    }

    return ans + nums[0];
  }
};

int main() {
  vector<tuple<vector<int>, int, int, long long>> tests{
      {{1, 3, 2, 6, 4, 2}, 3, 3, 5},
      {{10, 1, 2, 2, 2, 1}, 4, 3, 15},
      {{10, 8, 18, 9}, 3, 1, 36},
  };

  for (auto&[nums, k, dist, ans] : tests) {
    assert(Solution().minimumCost(nums , k , dist) == ans);
  }
}