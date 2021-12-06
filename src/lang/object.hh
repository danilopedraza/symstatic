#ifndef OBJECT_HH
#define OBJECT_HH

#include "ast.hh"
#include "libraries.hh"

namespace Object {

    enum class ObjectType: int
    {
        Boolean,
        Function,
        Integer
    };

    class Object
    {
    public:
        ObjectType type;
        virtual std::wstring str() = 0;
    };

    class Boolean: public Object
    {
    public:
        Boolean(bool value);
        std::wstring str();
        bool value;
    };

    class Function: public Object
    {
    private:
        ASTNode *function;
    public:
        Function(ASTNode* function);
        ASTNode *getFunction();
        std::wstring str();
    };

    class Integer: public Object
    {
    public:

        Integer(long long value);
        std::wstring str();
        long long value;
    };
}

#endif
