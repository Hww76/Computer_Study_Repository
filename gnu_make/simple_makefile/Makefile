include ./dir/makefile
INCLUDES = -I./include/

hello : hello.o
	gcc -o hello hello.o

hello.o : hello.c
	gcc -c hello.c -o hello.o $(INCLUDES)

.PHONY : clean # 将clean当作伪目标，不会因为有同名的clean文件而不执行clean操作
clean :
	rm hello
	rm *.o

print : 
	@echo $(.INCLUDE_DIRS)
	@echo $(subMakefile)
