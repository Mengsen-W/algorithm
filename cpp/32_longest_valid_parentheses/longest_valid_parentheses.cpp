#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    int longestValidParentheses(string s) {
        int left = 0, right = 0, maxlength = 0;
        for (int i = 0; i < s.length(); i++) {
            if (s[i] == '(') {
                left++;
            } else {
                right++;
            }
            if (left == right) {
                maxlength = max(maxlength, 2 * right);
            } else if (right > left) {
                left = right = 0;
            }
        }
        left = right = 0;
        for (int i = (int)s.length() - 1; i >= 0; i--) {
            if (s[i] == '(') {
                left++;
            } else {
                right++;
            }
            if (left == right) {
                maxlength = max(maxlength, 2 * left);
            } else if (left > right) {
                left = right = 0;
            }
        }
        return maxlength;
    }
};

int main() {
  vector<tuple<string, int>> tests{
      {"(()", 2},
      {")()())", 4},
      {"", 0},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().longestValidParentheses(s) == expected);
  }
  return 0;
}