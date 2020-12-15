# Java-多线程

~~~java
public class ReflectionStudy {
    public static void main(String args[]) {
        Thread thread = new myThread();
        thread.start();
    }
}
class myThread extends Thread{
    @Override
    public void run() {
        System.out.println("start new thread");
    }
}
~~~

我们从`Thread`派生了一个子类，然后覆盖重写`run`方法，当执行`thread.start()`的时候可以自动调用run方法。

~~~java
public class ReflectionStudy {
    public static void main(String args[]) {
        Thread thread = new Thread(new myThread());
        thread.start();
    }
}
class myThread implements Runnable{
    @Override
    public void run() {
        System.out.println("start new thread");
    }
}
~~~

这是另外的一种写法，此时我们创建的是一个`Thread`实例，传入的是一个`Runnable`实例。此时我们是通过`Runnable`接口来实现创建线程。

~~~java
public class ReflectionStudy {
    public static void main(String args[]) {
        Thread t = new Thread(()->{
            System.out.println("new thread");
        });
        t.start();
    }
}
~~~

Java8引入了`lambda`语法，代码编写更加简单化。

~~~java
import com.sun.tools.javac.code.Attribute;
import com.sun.xml.internal.ws.policy.privateutil.PolicyUtils;

public class ReflectionStudy {
    public static void main(String args[]) {
        System.out.println("main start");
        Thread t = new Thread(){
            public void run(){
                System.out.println("thread run....");
                try {
                    Thread.sleep(10);
                }catch (InterruptedException e){
                    e.printStackTrace();
                }
                System.out.println("thread end");
            }
        };
        t.start();
        try {
            Thread.sleep(10);//其参数是毫秒
        }catch (InterruptedException e){
            e.printStackTrace();
        }
        System.out.println("main end");
    }
}
~~~

记得不要直接调用run方法，这样是无效的。因为直接调用`run`方法，相当于调用了一个普通的类方法，当前线程没有任何改变，也不会启动新的线程。

> 必须调用`Thread`实例的`start()`方法才可以启动新线程，如果我们查看`Thread`类的源代码，会看到`start()`方法内部调用了一个`private native void start0()`方法，`native`修饰符表示这个方法是由JVM虚拟机内部C代码实现的，不是`java`代码实现的。

`线程的优先级`

~~~java
Thread.setPriority(int n);// 1-10 默认为5
~~~

