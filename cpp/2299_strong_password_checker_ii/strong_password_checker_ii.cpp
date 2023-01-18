/*
 * @Date: 2023-01-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-19
 * @FilePath: /algorithm/cpp/2299_strong_password_checker_ii/strong_password_checker_ii.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <unordered_set>

using namespace std;

class Solution {
 public:
  bool strongPasswordCheckerII(string password) {
    if (password.size() < 8) {
      return false;
    }

    unordered_set<char> specials = {'!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+'};
    int n = password.size();
    bool hasLower = false, hasUpper = false, hasDigit = false, hasSpecial = false;
    for (int i = 0; i < n; ++i) {
      if (i != n - 1 && password[i] == password[i + 1]) {
        return false;
      }

      char ch = password[i];
      if (islower(ch)) {
        hasLower = true;
      } else if (isupper(ch)) {
        hasUpper = true;
      } else if (isdigit(ch)) {
        hasDigit = true;
      } else if (specials.count(ch)) {
        hasSpecial = true;
      }
    }

    return hasLower && hasUpper && hasDigit && hasSpecial;
  }
};

int main() {
  assert(Solution().strongPasswordCheckerII("IloveLe3tcode!"));
  assert(!Solution().strongPasswordCheckerII("Me+You--IsMyDream"));
  assert(!Solution().strongPasswordCheckerII("1aB!"));
}