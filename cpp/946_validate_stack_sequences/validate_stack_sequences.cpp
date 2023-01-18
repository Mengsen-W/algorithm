/*
 * @Date: 2022-08-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-31
 * @FilePath: /algorithm/946_validate_stack_sequences/validate_stack_sequences.cpp
 */

#include <cassert>
#include <stack>
#include <vector>

using namespace std;

class Solution {
 public:
  bool validateStackSequences(vector<int>& pushed, vector<int>& popped) {
    stack<int> st;
    int n = pushed.size();
    for (int i = 0, j = 0; i < n; i++) {
      st.emplace(pushed[i]);
      while (!st.empty() && st.top() == popped[j]) {
        st.pop();
        j++;
      }
    }
    return st.empty();
  }
};

int main() {
  {
    vector<int> pushed{1, 2, 3, 4, 5};
    vector<int> popped{4, 5, 3, 2, 1};
    assert(Solution().validateStackSequences(pushed, popped));
  }
  {
    vector<int> pushed{1, 2, 3, 4, 5};
    vector<int> popped{4, 3, 5, 1, 2};
    assert(!Solution().validateStackSequences(pushed, popped));
  }
}