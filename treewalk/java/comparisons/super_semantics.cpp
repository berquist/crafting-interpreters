#include <iostream>

class A {
public:
  void method() {
    std::cout << "A method" << std::endl;
  }
};

class intermediate : public A {};

// class B : public A {
class B : public intermediate {
public:
  void method() {
    std::cout << "B method" << std::endl;
  }

  void test() {
    // A::method();
    intermediate::method();
  }
};

class C : public B {};

int main() {
  C().test();
  return 0;
}
