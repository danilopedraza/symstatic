#ifndef OBJECT_HH
#define OBJECT_HH

#include "libraries.hh"

namespace Object {

    enum class ObjectType: int
    {
        Integer
    };

    class Object
    {
    public:
        ObjectType type;
        virtual std::wstring str() = 0;
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
