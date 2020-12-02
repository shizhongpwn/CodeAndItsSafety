## C++primer-第七章-类

## this

this形参是隐式定义的。任何自定义名为`this`的参数或者变量行为都是非法的，它是一个常量指针。

成员还是在被调用的时候，会有一个`this`指针来访问调用它的那个对象。

`函数的类外定义`

~~~c++
class book{
    int add(int a,int b);
};
int book::add(int a, int b) {
    return a+b;
}
~~~

`默认构造函数`

~~~c++
class book{
    book() = default;
    int add(int a,int b);
};
~~~

构造函数不能被声明成`const`类型，当我们创建一个类的`const`对象的时候，直到构造函数初始化完成，对象才可以真正的取得`常量`属性，因此`构造函数`在初始化过程中可以对`const`对象写值。

当我们声明了一个构造函数的时候，编译器不再自动的生成默认构造函数，我们需要显示的成明出来。

~~~c++
book() = default;
~~~

`构造函数初始值列表`

~~~c++
    Sales_data(string &s):bookNo(s){};
~~~

当某个数据成员被构造函数初始值列表忽略的时候，它将以默认构造函数相同的方式进行隐式初始化。

`class和struct`的差别：

唯一的区别在于访问权限控制

* 类可以在它的第一个访问说明符（public , private）之前定义成员，对这种成员的访问方式依赖于类的定义方式。
* 使用`struct`那么第一个访问说明符之前的成员是`public`的，但是`claa`的话是`private`的

> 那么我们在编程的时候如果希望类的所有成员是`public`的时候使用`struct`,希望成员是`private`那么就是`class`.

`友元`

~~~c++
class Sales_data{
    friend Sales_data add(const Sales_data&,const Sales_data&);
    friend std::istream &read(std::istream&,Sales_data&);
    friend std::ostream &print(std::ostream&,const Sales_data&);

public:
    Sales_data() = default;
    Sales_data(const string &s,unsigned n,double p);
    Sales_data(string &s):bookNo(s){};
    Sales_data(std::istream&);
    std::string isbn() const {return bookNo};
    Sales_data &combine(const Sales_data&);

private:
    string bookNo;
    unsigned units_sold = 0;
    double revenue = 0.0;
};
Sales_data add(const Sales_data&,const Sales_data&);
std::istream &read(std::istream&,Sales_data&);
std::ostream &print(std::ostream&,const Sales_data&);
~~~

可以看到这些`add,read,print`函数可以访问类的私有成员，`友元`不是类的成员也不受它所在区域的访问控制权限的限制。









