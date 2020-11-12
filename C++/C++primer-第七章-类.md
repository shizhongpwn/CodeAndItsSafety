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

`class和struct`的差别：

唯一的区别在于访问权限控制









