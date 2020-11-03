## C++Primer-第二章

## const

其表示变量为常量，任何试图为改常量赋值的行为都会引发错误。

const的引用：

~~~c++
int c = 10;
const int &r1 = c;
~~~

引用及其对应的对象都是常量。

对const的引用可能引用一个非const对象：

~~~c++
int i=42;
int &r1 = i; //这里是一个普通引用
const int &r2 = i; //r2绑定非常量属于合法行为。但是不能通过r2来修改i的值。
r1 = 0; //正确
r2 = 0; //错误
~~~

指针和const:

这个和引用const十分相似，一句话：`指向常量的指针不能用于修改其所指向的对象的值，要想存放常量对象的地址，只能使用指向常量的指针`。

~~~c++
const double pi = 3.14;
double *ptr = &pi; \\错误，不是const指针
const double *ptr = &pi; \\正确
*ptr = 32; \\ 错误，const指针不能修改值，类比const 引用
~~~

但是值得注意的是const 指针可以指向非常量，但是不能用于修改其值。

const指针：

**指针是一个对象而引用却不是，这就是两者之间的差别**，所以我们可以使得指针本身设置为常量。

常量指针必须初始化，而且初始化完成不能修改，简单说就是指向固定地址了。

~~~c++
    int c = 10;
    int *const d = &c;  //注意书写的方式，*放在const的前面，用于表示指针才是常量，而指向的值可不一定奥。
        c = 20; // c还是可以修改的
~~~

顶层const和底层const:

顶层const:指针本身是一个常量；底层const:指针指向的对象是一个常量。

**constexpr和常量表达式**：

`常量表达式`是指值不会改变，并且在编译的过程中就能得到计算结果的表达式。

一个对象（或者表达式）是不是常量表达式是有其数据类型和初始值共同决定的：

~~~c
  const int max_files = 20;  //常量表达式
    const int limit = max_files + 1; //常量表达式
~~~

constexpr变量：

> C++11 新标准规定，允许将将变量声明为constexpr类型，以便由编译器来验证变量的值是否是一个常量表达式，声明为`constexpr`变量一定是一个常量，切必须用常量表达式初始化。

~~~c++
constexpr int mf = 20l;
constexpr int limit = mf + 1;
constexpr int sz = size();  //只有当size是一个constexpr函数时，才是一条正确的声明语句。
~~~

