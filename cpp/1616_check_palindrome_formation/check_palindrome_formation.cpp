/*
 * @Date: 2023-03-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-18
 * @FilePath: /algorithm/cpp/1616_check_palindrome_formation/check_palindrome_formation.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool checkSelfPalindrome(const string &a, int left, int right) {
    while (left < right && a[left] == a[right]) {
      left++;
      right--;
    }
    return left >= right;
  }

  bool checkConcatenation(const string &a, const string &b) {
    int n = a.size();
    int left = 0, right = n - 1;
    while (left < right && a[left] == b[right]) {
      left++;
      right--;
    }
    if (left >= right) {
      return true;
    }
    return checkSelfPalindrome(a, left, right) || checkSelfPalindrome(b, left, right);
  }

  bool checkPalindromeFormation(string a, string b) { return checkConcatenation(a, b) || checkConcatenation(b, a); }
};

int main() {
  {
    string a = "x";
    string b = "y";
    bool ans = true;
    assert(Solution().checkPalindromeFormation(a, b) == ans);
  }

  {
    string a = "abdef";
    string b = "fecab";
    bool ans = true;
    assert(Solution().checkPalindromeFormation(a, b) == ans);
  }

  {
    string a = "ulacfd";
    string b = "jizalu";
    bool ans = true;
    assert(Solution().checkPalindromeFormation(a, b) == ans);
  }
}