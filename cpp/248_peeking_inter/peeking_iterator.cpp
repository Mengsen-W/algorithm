/*
 * @Date: 2021-10-05 18:08:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-05 18:15:03
 */

/*
 * Below is the interface for Iterator, which is already defined for you.
 * **DO NOT** modify the interface for Iterator.
 */

#include <vector>

using namespace std;

class Iterator {
  struct Data;
  Data* data;

 public:
  Iterator(const vector<int>& nums);
  Iterator(const Iterator& iter);

  // Returns the next element in the iteration.
  int next();

  // Returns true if the iteration has more elements.
  bool hasNext() const;
};

template <class T>
class PeekingIterator : public Iterator<T> {
 public:
  PeekingIterator(const vector<T>& nums) : Iterator<T>(nums) {
    flag = Iterator<T>::hasNext();
    if (flag) {
      nextElement = Iterator<T>::next();
    }
  }

  T peek() { return nextElement; }

  T next() {
    T ret = nextElement;
    flag = Iterator<T>::hasNext();
    if (flag) {
      nextElement = Iterator<T>::next();
    }
    return ret;
  }

  bool hasNext() const { return flag; }

 private:
  T nextElement;
  bool flag;
};
