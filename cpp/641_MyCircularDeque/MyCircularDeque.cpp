/*
 * @Date: 2022-08-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-15
 * @FilePath: /algorithm/641_MyCircularDeque/MyCircularDeque.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class MyCircularDeque {
 private:
  vector<int> elements;
  int rear, front;
  int capacity;

 public:
  MyCircularDeque(int k) {
    capacity = k + 1;
    rear = front = 0;
    elements = vector<int>(k + 1);
  }

  bool insertFront(int value) {
    if (isFull()) {
      return false;
    }
    front = (front - 1 + capacity) % capacity;
    elements[front] = value;
    return true;
  }

  bool insertLast(int value) {
    if (isFull()) {
      return false;
    }
    elements[rear] = value;
    rear = (rear + 1) % capacity;
    return true;
  }

  bool deleteFront() {
    if (isEmpty()) {
      return false;
    }
    front = (front + 1) % capacity;
    return true;
  }

  bool deleteLast() {
    if (isEmpty()) {
      return false;
    }
    rear = (rear - 1 + capacity) % capacity;
    return true;
  }

  int getFront() {
    if (isEmpty()) {
      return -1;
    }
    return elements[front];
  }

  int getRear() {
    if (isEmpty()) {
      return -1;
    }
    return elements[(rear - 1 + capacity) % capacity];
  }

  bool isEmpty() { return rear == front; }

  bool isFull() { return (rear + 1) % capacity == front; }
};

int main() {
  MyCircularDeque m{3};
  assert(m.insertLast(1) == true);
  assert(m.insertLast(2) == true);
  assert(m.insertFront(3) == true);
  assert(m.insertFront(4) == false);
  assert(m.getRear() == 2);
  assert(m.isFull() == true);
  assert(m.deleteLast() == true);
  assert(m.insertFront(4) == true);
  assert(m.getFront() == 4);
}