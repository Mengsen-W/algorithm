/*
 * @Date: 2023-03-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-17
 * @FilePath: /algorithm/cpp/2389_answer_queries/answer_queries.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> answerQueries(vector<int>& nums, vector<int>& queries) {
    int n = nums.size(), m = queries.size();
    sort(nums.begin(), nums.end());
    vector<int> f(n + 1);
    for (int i = 0; i < n; i++) {
      f[i + 1] = f[i] + nums[i];
    }
    vector<int> answer(m);
    for (int i = 0; i < m; i++) {
      answer[i] = upper_bound(f.begin(), f.end(), queries[i]) - f.begin() - 1;
    }
    return answer;
  }
};

int main() {
  {
    vector<int> nums{4, 5, 2, 1};
    vector<int> queries{3, 10, 21};
    vector<int> ans{2, 3, 4};
    assert(Solution().answerQueries(nums, queries) == ans);
  }

  {
    vector<int> nums{2,3,4,5};
    vector<int> queries{1};
    vector<int> ans{0};
    assert(Solution().answerQueries(nums, queries) == ans);
  }
}