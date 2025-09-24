#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int check(int x, vector<int>& A, vector<int>& B, int n) {
    int rotations_a = 0, rotations_b = 0;
    for (int i = 0; i < n; i++) {
      // rotations coudn't be done
      if (A[i] != x && B[i] != x) return -1;
      // A[i] != x and B[i] == x
      else if (A[i] != x)
        rotations_a++;
      // A[i] == x and B[i] != x
      else if (B[i] != x)
        rotations_b++;
    }
    // min number of rotations to have all
    // elements equal to x in A or B
    return min(rotations_a, rotations_b);
  }

  int minDominoRotations(vector<int>& A, vector<int>& B) {
    int n = A.size();
    int rotations = check(A[0], B, A, n);
    // If one could make all elements in A or B equal to A[0]
    if (rotations != -1 || A[0] == B[0]) return rotations;
    // If one could make all elements in A or B equal to B[0]
    else
      return check(B[0], B, A, n);
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{2, 1, 2, 4, 2, 2}, {5, 2, 6, 2, 3, 2}, 2},
      {{3, 5, 1, 2, 3}, {3, 6, 3, 3, 4}, -1},
  };

  for (auto& [a, b, ans] : tests) {
    assert(Solution().minDominoRotations(a, b) == ans);
  }
}