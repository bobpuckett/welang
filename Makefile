default:
	# Set up
	clear
	mkdir -p build
	mkdir -p temp
	# Make source files
	bison -o temp/we.tab.c -d src/we.y 
	flex -o temp/lex.yy.c src/we.l
	# Compile source 
	gcc -o build/we temp/we.tab.c temp/lex.yy.c -lfl \
		`llvm-config --cflags --ldflags --libs core orcjit native`
	# Clean up folder
	rm -rf temp

