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
* 使用`struct`那么第一个访问说明符之前的成员是`public`的，但是`class`的话是`private`的

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

### 类的其他特性

~~~c++
#include<iostream>
using namespace std;
class Screen{
public:
    typedef string::size_type pos; //通过把pos定义成public成员可以隐藏Screen的实现细节
    Screen() = default; //我们提供了一个构造函数，编译器不会自动生成默认的构造函数，如果需要必须显示的声明出来。
    Screen(pos ht,pos wd,char c):height(ht),width(wd),contents(ht *wd,c){} //cursor成员已经显示的初始化为0
    char get() const {return contents[cursor];}
    inline char get(pos ht,pos wd) const ;
    Screen &move(pos r,pos c);
private:
    pos cursor = 0;
    pos height = 0,width = 0;
    string  contents;
};

~~~

* 定义在类内部的成员函数默认是`inline`的。

`可变数据成员`

* `mutable`关键字

一个可变数据成员不会是`const`的，即使它是`const`对象的成员。

~~~c++
class test{
public:
    void some_member() const ;
    void print(){
        cout<<access_ctr<<endl;
    }
private:
    mutable size_t access_ctr = 0;
};
void test::some_member() const {
    ++access_ctr;
}
int main()
{
    test a;
    a.some_member();
    a.print();
    return 0;
}
~~~

可以看到尽管函数为`const`标识的`只读`函数，但是其依然可以修改可变数据成员。

`类初始值`

~~~c++
class Window_mgr{
private:
    vector<Screen> screens{Screen(24,80,' ')};
};
~~~

这个好理解，就是使用类套用实现初始化了。

`返回*this的成员函数`

~~~c++
inline Screen &Screen::set(char c) {
    contents[cursor] = c;
    return *this;
}
inline Screen &Screen::set(pos r, pos col, char ch) {
    contents[r*width + col] = ch;
    return *this;
}
~~~

set成员函数返回的是调用`set`的对象的引用。如果没有`&`那么将会返回一个对象的副本。

`从const成员函数返回*this`

成员函数为`const`，那么返回的`this`指针就是`const`类型，其对应的对象也是`const`类型的。

`基于const的重载`

区别于上面的`const`类型返回值的成员函数，加入成员函数本事是`const`的，那么它就会实现重载，同时：

* 非常量版本的函数对常量对象是不可用的。
* 可以再非常量对象上调用常量版本或者非常量版本。

常量函数：

~~~c++
    inline char get(pos ht,pos wd) const ;
~~~

const在后面。

`类的声明`

前向声明:

~~~c++
class Link_Screen;
~~~

如果没有对其定义，我们只能有以下操作：

* 定义指向这种类型的引用或者指针。
* 也可以声明（不能定义）使用该`不完全类型`作为参数或者返回值的函数。

因为编译器并不知道它的空间大小和成员。

其好处在于**一个类的名字出现之后，它被认为是声明过的，此时此类可以允许包含指向自身类型的指针或者引用。**

~~~c++
class Link_Screen{
	Screen window;
	Link_Screen *next;
	Link_Screen *prev;
};
~~~



























