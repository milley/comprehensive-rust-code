#include <memory>
#include <iostream>
#include <string>

class Person {
public:
    Person() {
        std::cout << "Person Construction" << std::endl;
    }
    ~Person() {
        std::cout << "Person Destructor" << std::endl;
    }

    std::string name;
};

void say_hello(std::unique_ptr<Person> person) {
    std::cout << "Hello " << person->name << std::endl;
}

int main() {
    std::unique_ptr<Person> person = std::make_unique<Person>();
    person.get()->name = "John";

    say_hello(std::move(person));

    return 0;
}
