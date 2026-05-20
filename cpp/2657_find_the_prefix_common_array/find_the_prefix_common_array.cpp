#include <bit>
#include <cassert>
#include <cstdint>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> findThePrefixCommonArray(vector<int>& A, vector<int>& B) {
    uint64_t p = 0, q = 0;
    for (int i = 0; i < A.size(); i++) {
      p |= 1ULL << A[i];
      q |= 1ULL << B[i];
      A[i] = popcount(p & q);
    }
    return A;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, vector<int>>> tests{
      {{1, 3, 2, 4}, {3, 1, 2, 4}, {0, 2, 3, 4}},
      {{2, 3, 1}, {3, 1, 2}, {0, 1, 3}},
  };

  for (auto& [A, B, expect] : tests) {
    assert(Solution().findThePrefixCommonArray(A, B) == expect);
  }
}