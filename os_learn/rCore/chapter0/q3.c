// 在Linux环境下编写一个可以睡眠5秒后打印出一个字符串，
// 并把字符串内容存入一个文件中的应用程序A。(基于C或Rust语言)

#include <stdio.h>
#include <unistd.h>

int main() {
    sleep(5);

    const char* hello_string = "Hello Linux!\n";

    printf(hello_string);

    FILE *output_file = fopen("output.txt", "w");
    fputs(hello_string, output_file);
    fclose(output_file);

    return 0;
}