include ./dir/makefile # 引入子文件
INCLUDES = -I./include/ # 自定义头文件夹
export HELLO = hi # 自定义环境变量

hello : hello.o
	gcc -o hello hello.o

hello.o : hello.c
	gcc -c hello.c -o hello.o $(INCLUDES)

.PHONY : clean # 将clean当作伪目标，不会因为有同名的clean文件而不执行clean操作
clean :
	rm hello
	rm *.o

print : 
	@echo $(.INCLUDE_DIRS) # 打印系统头文件地址
	@echo $(subMakefile) # 打印子文件中的变量
	@echo $(USER) # 打印系统环境变量
	@echo $(HELLO) # 打印自定义环境变量
