.SECONDEXPANSION: #添加二阶段展开关键词
VPATH = src:headers

obj = $(wildcard *.c) # wildcard 是关键字，能将通配符展开,否则就是原样的*.c
objs = hello


hello: hello.o
	gcc hello.o -o hello

hello.o: hello.c | hello.h
	gcc -c $< -o hello.o -Iheaders # $< 作用：寻找第一个依赖，会自动补全路径

.PHONY: print
print: $$(objs) # 第一阶段将$$(objs)的第1个$生效，变为了$(objs)，然后objs被二次赋值，当二次展开时的$(obj)的值为后一次赋值内容
	@echo $^

objs = hello.o

clean:
	rm hello
	rm -f *.o
