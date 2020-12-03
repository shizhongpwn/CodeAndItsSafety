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
const int &r2 = i; //r2绑定非常量属于合法行为。但是不能通过r2来修改i的值,这和顶层const指针十分相似。
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

`const函数`

经常见到的是，一写类成员函数后加const，那么表明该函数是`只读`函数，其不会修改类内成员变量。

如果`const`加在开始的地方，那么表示返回值const类型。

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

constexpr指针：

这个其实和顶层const有点类似：

~~~c++
const int *p = nullptr; //p是一个指向整形常量的指针
constexpr int *q = nullptr; //q是一个指向整数的常量指针
~~~

## 类型别名

传统的方法就是`typedef`了，但是新标准规定可以使用`别名声明`来定义类型的别名：

~~~c++
using SI = Sales_items;  //SI 和 Sales_items相等
~~~

## auto

> c++11 引入了`auto`类型说明符，它可以让编译器自动判断表达式所属的类型，`auto`通过初始值来推算变量的类型，所以,`auto`定义的变量必须有初始值。

~~~c++
auto i = 0,*p = &i; //正确
auto sz = 0,pi = 3.14 ; //错误，sz和pi的类型不同
~~~

复合类型，常量和auto:

编译器推算出来的auto类型有时候和初始值得类型并不是完全一样的，编译器会适当的改变结果使得其更符合初始化规则。
比如引用：

~~~c++
int i=0,&r = i;
auto a = r; //a是一个整数（r是i的别名，i是一个整数）
~~~

const:

auto一般会忽略掉顶层const，同时底层const则会保留下来，比如初始值是一个指向常量的指针：

~~~c++
const int i = 1;
const int ci = i,&cr = ci;
auto b = ci; // b是一个整数类型，其顶层const性质被忽略
auto c =  cr; // c是一个整数，cr是ci的别名，ci的顶层const被忽略
auto d = &i; // d是一个指针
auto e = &ci; //e是一个指向整数常量的指针（对常量的取地址引用是一种底层const） 
~~~

那么如何`auto`推算出顶层const?，答案是提前声明：

~~~c++
const auto f = ci; //ci是int f 是const int
~~~

或者我们可以使用引用：

~~~c++
auto &g = ci; //ci 是一个整形常量，g是一个整形常量的引用；
auto &h = 52; //错误，不能为非常量引用绑定字面值。 const &h = 52;就可
~~~

## decltype类型指示符

> c++11新标准里面引入了类型说明符`decltype`，它的作用是返回操作数类型，在此过程中，编译器法分析表达式并得到它的类型，却不实际计算他得到的值。

~~~c++
decltype (f()) sum = x;
~~~

编译器并不实际调用函数f，而是使用当调用发生时f的返回值类型作为sum的类型。编译器指定的类型为：假如f函数被调用的话将会是其返回值得类型。

**其解决的问题在于：有时候我们希望表达式推断出要定义的变量的类型，但是不想用改表达式的值来初始化这个变量。**

处理`const`:

~~~c++
const int ci=0,&cj=ci;
decltype(ci) x = 0; //x是一个顶层const类型
decltype(cj) y = x; //y是const int &,y绑定到x上；
decltype(cj) z;  // cj 是一个引用，必须初始化。
~~~

通过上面就可以说明，和`auto`不同，`decltype`返回变量类型的时候包含顶层const和引用。

decltype和引用：

~~~c++
int i=52. *p=&i,&r = i;
decltype(r + 0) b; // 加法的结果是int ,所以b是int 类型
decltype(r) d;  //此时就是一个引用类型，错误，引用必须初始化
decltype(*p) c; // *p 对应的是一个引用，所以c是一个引用，错误，引用必须初始化。
~~~

值得注意的是`解引用`操作对应的是一个引用类型，解引用得到的是指针指向的对象，还可以修改改对象，所以得到的是引用类型，而不是int类型。

括号的影响：

~~~c++
decltype(i) b; //这是一个整形。
decltype((r)) b; // 这是一个引用类型
~~~

> 其余剩下的就跟C语言几乎一样了，就不做额外做记录了。



















