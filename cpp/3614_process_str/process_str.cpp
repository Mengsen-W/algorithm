#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  char processStr(string s, long long k) {
    long long len = 0;
    for (auto c : s) {
      if (c == '*') {
        if (len) {
          len--;
        }
      } else if (c == '#') {
        len *= 2;
      } else if (c == '%') {
        continue;
      } else {
        len++;
      }
    }
    if (k + 1 > len) {
      return '.';
    }
    for (int i = s.size() - 1; i >= 0; i--) {
      if (s[i] == '*') {
        len++;
      } else if (s[i] == '#') {
        if (k + 1 > (len + 1) / 2) {
          k -= len / 2;
        }
        len = (len + 1) / 2;
      } else if (s[i] == '%') {
        k = len - k - 1;
      } else {
        if (k + 1 == len) {
          return s[i];
        } else {
          len--;
        }
      }
    }
    return '.';
  }
};

int main() {
  vector<tuple<string, long long, char>> tests{
      {"a#b%*", 1, 'a'},
      {"cd%#*#", 3, 'd'},
      {"z*#", 0, '.'},
  };

  for (auto [s, k, expected] : tests) {
    auto result = Solution().processStr(s, k);
    if (result != expected) {
      printf("failed: %s, %lld, %c, %c\n", s.c_str(), k, expected, result);
    }
  }
}