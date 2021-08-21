/*
 * @Date: 2021-08-21 14:13:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-21 14:54:32
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int compress(vector<char>& chars) {
    int n = chars.size();
    int write = 0, left = 0;
    for (int read = 0; read < n; read++) {
      if (read == n - 1 || chars[read] != chars[read + 1]) {
        chars[write++] = chars[read];
        int num = read - left + 1;
        if (num > 1) {
          int anchor = write;
          while (num > 0) {
            chars[write++] = num % 10 + '0';
            num /= 10;
          }
          reverse(&chars[anchor], &chars[write]);
        }
        left = read + 1;
      }
    }
    chars.resize(write);
    return write;
  }
};

int main() {
  {
    vector<char> chars{'a', 'a', 'b', 'b', 'c', 'c', 'c'};
    vector<char> ans{'a', '2', 'b', '2', 'c', '3'};
    assert(Solution().compress(chars) == 6);
    assert(chars == ans);
  }
  {
    vector<char> chars{'a'};
    vector<char> ans{'a'};
    assert(Solution().compress(chars) == 1);
    assert(chars == ans);
  }
  {
    vector<char> chars{'a', 'b', 'b', 'b', 'b', 'b', 'b',
                       'b', 'b', 'b', 'b', 'b', 'b'};
    vector<char> ans{'a', 'b', '1', '2'};
    assert(Solution().compress(chars) == 4);
    assert(chars == ans);
  }
}