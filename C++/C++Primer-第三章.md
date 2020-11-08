# C++Primer-第三章

## 命名空间using声明

比如`std::cin`这其实表示的是：编译器应从操作符的左侧名字所示的作用域中寻找右侧那个名字。

值得注意的是，头文件不应该使用`using`声明，因为头文件的内容会拷贝到所有引用它的文件中去，这很可能导致冲突（比如命名冲突）。

## 标准库类型string

> 比较平常的不做记录

初始化方式：

~~~c++
string s1; //默认初始化，其为NULL
string s1 = s2;
string s1 = "abcd";
string s1(10,"c");
~~~

string::size_type类型：

~~~c++
string a = "asdad";
int b = a.size();
~~~

你以为`size()`函数返回的是`int`类型？其实不是，`size`函数返回的是一个`string::size_type`类型。

* 它一个无符号类型，足够放下任何`string`对象的大小，那么所有用于存放`size()`函数返回值的变量都应该是`string::size_type`类型的。

  * ~~~c++
    auto len = a.size(); //这样就可以啦，我们调试发现len的类型为unsigned int 但是其类型为`string::size_type`（书上这么说的，但是肯定是一个无符号类型整数啦）
    ~~~

string对象特性：

* `string`对象会自动忽略空白。

  * ~~~ c++
    string s;
    cin>>s;
    //输入 "     abcd"
    //s == "abcd"
    ~~~

* `string`的比较规则：

  * 判断两个`string`的长度是否相同。
  * 判断两个`string`的包含字符是否相等。

  > 这其实避免了C语言中`\x00`所带来的可能错误。

* `string`的相加规则

  * C++允许`string`和字面值相加。

    * ~~~c++
      string a = "asdsa";
      string b = a + "asda";
      ~~~

  * C++相加的时候必须保证`+`的两侧至少有一个`string`.

    * ~~~c++
      string a = "asdasd";
      string b = a + "asdsa";
      string c = "Asd" + "asdsa";//错误，+的两侧都是字面值
      string d = a + "asda" + "dasdas";//正确。因为其等于 string d = (a + "asda") + "dasdas";
      string e = "asdasd" + "dasdas" + a;//错误，还是出现了字面值相加
      ~~~

C++字符特性判断总结（这个还是比较常用的）：

~~~c++
    char d = 'a';
    isalnum(d); //当其是字母和或者数字时为真
    isalpha(d); // 当其为字母的时候为真
    iscntrl(d); // 当其为控制字符的时候为真
    isdigit(d); // 当其是数字的时候为真
    isgraph(d); //当其不是空格但是可打印的时候为真
    islower(d); //当其为小写字母的时候为真 
    isprint(d); // 当其是可打印字符的时候为真（包括空格，及其其他可视形式）
    ispunct(d); //当其是标点符号的时候为真
    isspace(d); //当c是空白的时候为真（包括空格，横向制表符，纵向制表符，回车符，换行符，进纸符（匹配一个换页符。等价于\x0c和\cL。）的一种）
    isupper(d); //当其是大写字母的时候为真
    isxdigit(d); //当其是十六进制的时候为真
    tolower(d); //d是大写字母则变成小写字母，否则原样输出
    toupper(d); //d是小写字母的时候变成大写字母，否则原样输出
~~~

`string`的字符处理：

~~~c++
 string s1 = "asdasd";
    for(auto s : s1)
    {
        cout<<s<<endl;
    }
~~~

for字符变换处理：

~~~c++
   string s1 = "asdasd";
    for(auto &s : s1)
    {
        s = 'a';
    }
    cout<<s1; //输出结果全是a
~~~

## 标准库类型`vector`

头文件`#include<vector>`

 `vector`初始化：

~~~c++
    vector<string> v1; //空的vector ，潜在类型为string,执行默认初始化。
    vector<string> v2(v1); // v2等于v1的副本
    vector<string> v3 = v1; // 同上
    vector<string> v4(10,"Asdsad"); // v4包含10个"Asdsad"
    vector<string> v8{123,"Asda"}; //显然不能用int初始化v8,所以无法执行列表初始化之后，编译器尝试用默认值初始化vector对象。
    vector<string> v5(10);  //v5包含10个执行默认初始化的值
    vector<string> v6{"asda","Asdsa","wqewq"}; // v6包含3个值为里面那样的元素，这是一种列表初始化的方式，注意{}
    vector<string> v7 = {"asdsad","Dsada","dasdsa"}; //跟v6类似
~~~

有时候，`vector`对象创建的时候并不知道其实际所需的元素的个数，一个理想情况就是：

1. 先创建一个空的vector对象
2. 通过`push_back`向里面添加元素。

~~~c++
    vector<int> v10;
    for (int i = 0; i <20; ++i) {
        v10.push_back(i);
    }
~~~

`vector`在使用的时候的注意事项：

* 

`vector`重要操作：

~~~c++
    vector<string> v8{123,"Asda"};
    v8.empty();
    v8.size(); //这个还是要注意他的返回值，vector<string>::size_type.这里就和string::size_type一样是个unsign int
    v8.push_back("sad");
    string a = v8[1];
~~~

![image-20201108205850637](C++Primer-第三章.assets/image-20201108205850637.png)

同时也存在`==`,!=这些常规操作。

其实对于`vector`里面元素的使用方法和其元素本身的类型相关：

~~~c++
    vector<int> v10;
    for (int i = 0; i <20; ++i) {
        v10.push_back(i);
    }
    for(auto &g:v10){
        g *= g;
    }
    for(auto i : v10){
        cout<<i<<endl;
    }
~~~

vector元素的比较：

* 两个vector相等当且仅当它们所含的元素个数相等，且对应位置上的元素的值也相等。

* 如果两个vector元素的个数不相等，但是相应位置上的元素相等，那么元素较少 的`vector`对象小于元素较多的`vector`对象。

  * 如果元素有却别，那么vector对象的大小关系由第一对相异的值的大小关系确定。

值得注意的是`vector`虽然允许使用下标访问元素，但是**禁止通过下标增加元素，同时下标必须合法。**

  

