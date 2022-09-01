/*
 * @Date: 2022-09-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-01
 * @FilePath: /algorithm/1475_final_prices/final_prices.cpp
 */

#include <cassert>
#include <stack>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> finalPrices(vector<int>& prices) {
    int n = prices.size();
    vector<int> ans(n);
    stack<int> st;
    for (int i = n - 1; i >= 0; i--) {
      while (!st.empty() && st.top() > prices[i]) {
        st.pop();
      }
      ans[i] = st.empty() ? prices[i] : prices[i] - st.top();
      st.emplace(prices[i]);
    }
    return ans;
  }
};

int main() {
  {
    vector<int> prices{8, 4, 6, 2, 3};
    vector<int> ans{4, 2, 4, 2, 3};
    assert(Solution().finalPrices(prices) == ans);
  }

  {
    vector<int> prices{1, 2, 3, 4, 5};
    vector<int> ans{1, 2, 3, 4, 5};
    assert(Solution().finalPrices(prices) == ans);
  }

  {
    vector<int> prices{10, 1, 1, 6};
    vector<int> ans{9, 0, 1, 6};
    assert(Solution().finalPrices(prices) == ans);
  }
}