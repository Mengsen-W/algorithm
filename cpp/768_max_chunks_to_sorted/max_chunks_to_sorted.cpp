/*
 * @Date: 2022-08-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-15
 * @FilePath: /algorithm/768_max_chunks_to_sorted/max_chunks_to_sorted.cpp
 */

#include <cassert>
#include <stack>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxChunksToSorted(vector<int> arr) {
    stack<int> st;
    for (auto& num : arr) {
      if (st.empty() || num >= st.top()) {
        st.emplace(num);
      } else {
        int mx = st.top();
        st.pop();
        while (!st.empty() && st.top() > num) {
          st.pop();
        }
        st.emplace(mx);
      }
    }
    return st.size();
  }
};

int main() {
  assert(Solution().maxChunksToSorted({5, 4, 3, 2, 1}) == 1);
  assert(Solution().maxChunksToSorted({2, 1, 3, 4, 4}) == 4);
}
