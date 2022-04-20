/*
 * @Date: 2022-04-20 09:16:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-20 09:28:24
 * @FilePath: /algorithm/388_length_longest_path/length_longest_path.cpp
 */

#include <cassert>
#include <stack>
#include <string>

using namespace std;

class Solution {
 public:
  int lengthLongestPath(string input) {
    int n = input.size();
    int pos = 0;
    int ans = 0;
    stack<int> st;

    while (pos < n) {
      /* 检测当前文件的深度 */
      int depth = 1;
      while (pos < n && input[pos] == '\t') {
        pos++;
        depth++;
      }
      /* 统计当前文件名的长度 */
      int len = 0;
      bool isFile = false;
      while (pos < n && input[pos] != '\n') {
        if (input[pos] == '.') {
          isFile = true;
        }
        len++;
        pos++;
      }
      /* 跳过换行符 */
      pos++;

      while (st.size() >= depth) {
        st.pop();
      }
      if (!st.empty()) {
        len += st.top() + 1;
      }
      if (isFile) {
        ans = max(ans, len);
      } else {
        st.emplace(len);
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().lengthLongestPath("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext") == 20);
  assert(Solution().lengthLongestPath("a") == 0);
  assert(Solution().lengthLongestPath("file1.txt\nfile2.txt\nlongfile.txt") == 12);
}
