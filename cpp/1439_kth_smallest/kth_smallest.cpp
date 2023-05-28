/*
 * @Date: 2023-05-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-28
 * @FilePath: /algorithm/cpp/1439_kth_smallest/kth_smallest.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int kthSmallest(vector<vector<int>>& mat, int k) {
    int m = mat.size();
    vector<int> prev = mat[0];
    for (int i = 1; i < m; ++i) {
      prev = merge(prev, mat[i], k);
    }
    return prev[k - 1];
  }

  vector<int> merge(const vector<int>& f, const vector<int>& g, int k) {
    int left = f[0] + g[0], right = f.back() + g.back(), thres = 0;
    k = min(k, static_cast<int>(f.size() * g.size()));
    while (left <= right) {
      int mid = (left + right) / 2;
      int rptr = g.size() - 1, cnt = 0;
      for (int lptr = 0; lptr < f.size(); ++lptr) {
        while (rptr >= 0 && f[lptr] + g[rptr] > mid) {
          --rptr;
        }
        cnt += rptr + 1;
      }
      if (cnt >= k) {
        thres = mid;
        right = mid - 1;
      } else {
        left = mid + 1;
      }
    }

    vector<int> ans;
    for (int i = 0; i < f.size(); ++i) {
      for (int j = 0; j < g.size(); ++j) {
        if (int sum = f[i] + g[j]; sum < thres) {
          ans.push_back(sum);
        } else {
          break;
        }
      }
    }
    while (ans.size() < k) {
      ans.push_back(thres);
    }
    sort(ans.begin(), ans.end());
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> mat = {{1, 3, 11}, {2, 4, 6}};
    int k = 5;
    int ans = 7;
    assert(Solution().kthSmallest(mat, k) == ans);
  }

  {
    vector<vector<int>> mat = {{1, 3, 11}, {2, 4, 6}};
    int k = 9;
    int ans = 17;
    assert(Solution().kthSmallest(mat, k) == ans);
  }

  {
    vector<vector<int>> mat = {{1, 10, 10}, {1, 4, 5}, {2, 3, 6}};
    int k = 7;
    int ans = 9;
    assert(Solution().kthSmallest(mat, k) == ans);
  }

  {
    vector<vector<int>> mat = {{1, 1, 10}, {2, 2, 9}};
    int k = 7;
    int ans = 12;
    assert(Solution().kthSmallest(mat, k) == ans);
  }
}