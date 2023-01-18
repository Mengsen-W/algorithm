/*
 * @Date: 2022-01-01 02:00:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-01 02:19:26
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> construct2DArray(vector<int> &original, int m, int n) {
    vector<vector<int>> ans;
    int original_size = original.size();
    if (original_size != m * n) {
      return ans;
    }
    for (auto it = original.begin(); it != original.end(); it += n) {
      ans.emplace_back(it, it + n);
    }
    return ans;
  }
};

int main() {
  {
    vector<int> original{1, 2, 3, 4};
    int m = 2;
    int n = 2;
    vector<vector<int>> ans{{1, 2}, {3, 4}};
    assert(Solution().construct2DArray(original, m, n) == ans);
  }

  {
    vector<int> original{1, 2, 3};
    int m = 1;
    int n = 3;
    vector<vector<int>> ans{{1, 2, 3}};
    assert(Solution().construct2DArray(original, m, n) == ans);
  }

  {
    vector<int> original{1, 2};
    int m = 1;
    int n = 1;
    vector<vector<int>> ans;
    assert(Solution().construct2DArray(original, m, n) == ans);
  }

  {
    vector<int> original{3};
    int m = 1;
    int n = 2;
    vector<vector<int>> ans;
    assert(Solution().construct2DArray(original, m, n) == ans);
  }
}