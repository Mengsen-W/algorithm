#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string breakPalindrome(string palindrome) {
    int n = palindrome.size();
    if (n == 1) {
      return "";
    }
    for (int i = 0; i * 2 + 1 < n; i++) {
      if (palindrome[i] != 'a') {
        palindrome[i] = 'a';
        return palindrome;
      }
    }
    palindrome.back()++;
    return palindrome;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"abccba", "aaccba"},
      {"a", ""},
  };

  for (auto &[palindrome, ans] : tests) {
    assert(Solution().breakPalindrome(palindrome) == ans);
  }
}