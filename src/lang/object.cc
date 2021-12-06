#include "libraries.hh"
#include "ast.hh"
#include "object.hh"

Object::Boolean::Boolean(bool value) {
    this->type = ObjectType::Boolean;
    this->value = value;
}
std::wstring Object::Boolean::str() {
    return this->value? L"verdadero" : L"falso";
}


Object::Function::Function(ASTNode* function) {
    this->type = ObjectType::Function;
    this->function = function;
}
std::wstring Object::Function::str() {
    return L"Objeto de tipo función";
}
ASTNode *Object::Function::getFunction() {return function;}


Object::Integer::Integer(long long value) {
    this->type = ObjectType::Integer;
    this->value = value;
}
std::wstring Object::Integer::str() {
    return std::to_wstring(this->value);
}
