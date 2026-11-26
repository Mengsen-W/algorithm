#include <algorithm>
#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int smallestUniqueSubarray(vector<int>& nums) {
    int n = (int)nums.size();

    vector<int> sa(n), rank(nums.begin(), nums.end());
    vector<int> nextRank(n);

    iota(sa.begin(), sa.end(), 0);

    // 倍增构造排序后的后缀数组
    int maxRank = *max_element(nums.begin(), nums.end());
    vector<int> temp(n), cnt(std::max(n + 1, maxRank + 2));

    auto counting_sort = [&](auto get_key) {
      fill(cnt.begin(), cnt.end(), 0);

      for (int i : sa) {
        ++cnt[get_key(i)];
      }

      for (int i = 1; i <= maxRank + 1; ++i) {
        cnt[i] += cnt[i - 1];
      }

      for (int p = n - 1; p >= 0; --p) {
        int i = sa[p];
        temp[--cnt[get_key(i)]] = i;
      }

      sa.swap(temp);
    };

    for (int k = 1; k < n; k <<= 1) {
      auto get_rank = [&](int i) -> pair<int, int> { return {rank[i], i + k < n ? rank[i + k] : -1}; };

      // 先按第二关键字排序，再按第一关键字稳定排序
      counting_sort([&](int i) { return get_rank(i).second + 1; });
      counting_sort([&](int i) { return get_rank(i).first + 1; });

      nextRank[sa[0]] = 0;

      // 给排序后的新后缀分配排名
      for (int i = 1; i < n; ++i) {
        auto rx = get_rank(sa[i - 1]), ry = get_rank(sa[i]);
        nextRank[sa[i]] = nextRank[sa[i - 1]] + (rx != ry);
      }

      rank.swap(nextRank);
      maxRank = rank[sa.back()];

      // n 个后缀有 n 个排名，排名各不相同，说明排序完毕
      if (maxRank == n - 1) {
        break;
      }
    }

    // Kasai 算法构造 LCP
    vector<int> pos(n), lcp(n, 0);
    for (int i = 0; i < n; ++i) {
      pos[sa[i]] = i;
    }

    int h = 0;
    for (int i = 0; i < n; ++i) {
      if (pos[i] == 0) {
        h = 0;
        continue;
      }

      int j = sa[pos[i] - 1];
      while (i + h < n && j + h < n && nums[i + h] == nums[j + h]) {
        ++h;
      }

      lcp[pos[i]] = h;

      if (h > 0) {
        --h;
      }
    }

    int answer = n;
    for (int i = 0; i < n; ++i) {
      int next = (i + 1 < n ? lcp[i + 1] : 0);

      int need = std::max(lcp[i], next) + 1;
      if (sa[i] + need - 1 < n) {
        answer = std::min(answer, need);
      }
    }

    return answer;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 3, 3}, 3},
      {{2, 1, 2, 3, 3}, 1},
      {{1, 1, 2, 2, 1}, 2},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().smallestUniqueSubarray(nums) == expected);
  }
}