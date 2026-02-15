#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string addBinary(string a, string b) {
    string result = "";
    int i = a.length() - 1, j = b.length() - 1;
    int carry = 0;

    while (i >= 0 || j >= 0 || carry) {
      int sum = carry;
      if (i >= 0) {
        sum += a[i--] - '0';
      }
      if (j >= 0) {
        sum += b[j--] - '0';
      }
      result = char(sum % 2 + '0') + result;
      carry = sum / 2;
    }

    return result;
  }
};

int main() {
  vector<tuple<string, string, string>> tests{
      {"11", "1", "100"},
      {"1010", "1011", "10101"},
  };

  for (auto& [a, b, ans] : tests) {
    assert(Solution().addBinary(a, b) == ans);
  }
}
