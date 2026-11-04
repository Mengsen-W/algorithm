#include <cassert>
#include <climits>
#include <set>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
    vector<long long> getCosts(const vector<int>& nums, int x) {
        int n = nums.size();
        vector<long long> ops(n);

        multiset<int> Left, Right;
        long long sumL = 0, sumR = 0;

        for (int i = 0; i < n; ++i) {
            int v = nums[i];
            if (Left.empty() || v <= *Left.rbegin()) {
                Left.insert(v);
                sumL += v;
            } else {
                Right.insert(v);
                sumR += v;
            }
            if (i >= x) {   
                v = nums[i - x];
                auto it = Left.find(v);
                if (it != Left.end()) {
                    sumL -= v;
                    Left.erase(it);
                } else {
                    it = Right.find(v);
                    sumR -= v;
                    Right.erase(it);
                }
            }
            if (Left.size() > Right.size() + 1) {
                auto it = prev(Left.end());
                v = *it;
                Right.insert(v);
                Left.erase(it);
                sumL -= v;
                sumR += v;
            } else if (Left.size() < Right.size()) {
                auto it = Right.begin();
                v = *it;
                Left.insert(v);
                Right.erase(it);
                sumL += v;
                sumR -= v;
            }
            if (i >= x - 1) {
                long long mid = *Left.rbegin();
                long long leftCost = mid * (long long)Left.size() - sumL;
                long long rightCost = sumR - mid * (long long)Right.size();
                ops[i] = leftCost + rightCost;
            }
        }
        return ops;
    }

public:
    long long minOperations(vector<int>& nums, int x, int k) {
        int n = nums.size();
        vector<long long> ops = getCosts(nums, x);
        vector<long long> pre(n + 1, 0);
        vector<long long> cur(n + 1, LLONG_MAX);

        for (int j = 1; j <= k; ++j) {
            fill(cur.begin(), cur.end(), LLONG_MAX);
            for (int i = j * x; i <= n; ++i) {
                cur[i] = min(cur[i - 1], pre[i - x] + ops[i - 1]);
            }
            pre.swap(cur);
        }
        return pre[n];
    }
};

int main() {
  vector<tuple<vector<int>, int, int, long long>> tests{
      {{5, -2, 1, 3, 7, 3, 6, 4, -1}, 3, 2, 8},
      {{9, -2, -2, -2, 1, 5}, 2, 2, 3},
  };

  for (auto [nums, x, k, expected] : tests) {
    assert(Solution().minOperations(nums, x, k) == expected);
  }
  return 0;
}