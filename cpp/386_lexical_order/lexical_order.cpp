#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> lexicalOrder(int n) {
    vector<int> ret(n);
    int number = 1;
    for (int i = 0; i < n; i++) {
      ret[i] = number;
      if (number * 10 <= n) {
        number *= 10;
      } else {
        while (number % 10 == 9 || number + 1 > n) {
          number /= 10;
        }
        number++;
      }
    }
    return ret;
  }
};

int main() {
  vector<tuple<int, vector<int>>> tests{
      {13, {1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9}},
      {2, {1, 2}},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().lexicalOrder(n) == ans);
  }
}