#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    long long maxSumTrionic(vector<int>& nums) {
        int n = nums.size();
        int p, q, j;
        long long max_sum, sum, res;
        long long ans = LLONG_MIN;
        for (int i = 0; i < n; i++) {
            j = i + 1;
            res = 0;
            //第一段
            for(; j < n && nums[j -1] < nums[j]; j++);
            p = j - 1;
            if (p == i) {
                continue;
            }
            //第二段
            res += nums[p] + nums[p - 1];
            for(; j < n && nums[j - 1] > nums[j]; j++){
                res += nums[j];
            }
            q = j - 1;
            if (q == p || q == n - 1 || (nums[j] <= nums[q])) {
                i = q;
                continue;
            }
            //第三段
            res += nums[q + 1];
            //第三段求累加最大值
            max_sum = 0;
            sum = 0;
            for (int k = q + 2; k < n && nums[k] > nums[k - 1]; k++) {
                sum += nums[k];
                max_sum = max(max_sum, sum);
            }
            res += max_sum;
            //第一段求累加最大值
            max_sum = 0;
            sum = 0;
            for (int k = p - 2; k >= i; k--) {
                sum += nums[k];
                max_sum = max(max_sum, sum);
            }
            res += max_sum;
            // 更新答案
            ans = max(ans, res);
            i = q - 1;
        }
        return ans;
    }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{0, -2, -1, -3, 0, 2, -1}, -4},
      {{1, 4, 2, 7}, 14},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().maxSumTrionic(nums) == ans);
  }
}