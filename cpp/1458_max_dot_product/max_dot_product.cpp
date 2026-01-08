#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    int maxDotProduct(vector<int>& nums1, vector<int>& nums2) {
        int m = nums1.size();
        int n = nums2.size();
        vector<vector<int>> f(m, vector<int>(n));

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                int xij = nums1[i] * nums2[j];
                f[i][j] = xij;
                if (i > 0) {
                    f[i][j] = max(f[i][j], f[i - 1][j]);
                }
                if (j > 0) {
                    f[i][j] = max(f[i][j], f[i][j - 1]);
                }
                if (i > 0 && j > 0) {
                    f[i][j] = max(f[i][j], f[i - 1][j - 1] + xij);
                }
            }
        }

        return f[m - 1][n - 1];
    }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{2, 1, -2, 5}, {3, 0, -6}, 18},
      {{3, -2}, {2, -6, 7}, 21},
      {{-1, -1}, {1, 1}, -1},
  };

  for (auto& [nums1, nums2, expected] : tests) {
    assert(Solution().maxDotProduct(nums1, nums2) == expected);
  }

}