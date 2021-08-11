#include "libraries.hh"
#include "object.hh"

Object::Integer::Integer(long long value) {
    this->value = value;
}

std::wstring Object::Integer::str() {
    return std::to_wstring(this->value);
}
