// 在Linux环境下编写一个会产生异常的应用程序，并简要解释操作系统的处理结果。

# include <stdio.h>

int main(){

    printf("1 / 0 = %d", 1/0);

    return 0;
}

/* 输出结果

q2.c: In function ‘main’:
q2.c:7:27: warning: division by zero [-Wdiv-by-zero]
    7 |     printf("1 / 0 = %d", 1/0);
      |                           ^
./q2
bash: ./q2: cannot execute binary file: Exec format error

*/