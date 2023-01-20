#include <stdlib.h>
#include <stdbool.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>
#include <pwd.h>
#include <string.h>

#include <pm/config.h>


char* make_line(const char* c){
	const char* c_cpy = c;
	while(*c_cpy != '\n'){
		c_cpy++;
	}

	size_t str_size = (c_cpy - c);
	char* line = calloc(sizeof(char), str_size);

	strncpy(line, c, str_size);

	return line;
}

int main(){
	//gets config directory
	//it should search it in /home/$USER/.config/lil_manager/config
	//but I am too lazy to do it rn
	FILE* f  = fopen("../config", "r");
	if(f == NULL){
		printf("could not find config director\n");
		return -1;
	}

	// this is the size that DOES NOT include the last 0
	size_t size = 0;
	fseek(f, 0L, SEEK_END);
	size = ftell(f);
	fseek(f, 0L, SEEK_SET);

	char* buffer = calloc(sizeof(char), size + 1);

	fread(buffer, size, 1, f);
	if(fclose(f)){
		printf("failed to close the file");
		return -1;
	}
	//just in case
	f = NULL;

	//TODO: preprocess the buffer
	//it should remove repeating \n\n
	//respect the whitespace inside " "
	//crash if the " is not closed


	//separates the string in lines
	//TODO: make it so it isn't just an array in the heap
	char** lines = calloc(sizeof(void*), 100);

	size_t index = 0;
	char prev = '\n';
	// vulnerability lmao but 
	// meh
	for(size_t i = 0; i < size; ++i){
		//TODO: this should push at the end of lines
		//instead of assuming that there is enought space
		if(prev == '\n'){
			lines[index++] = buffer + i;
		}
		prev = buffer[i];
	}

	// is this good practice?
	// like it pollutes the vector with a mixture of pointers that point to an address inside buffer*
	// or to an external address, there is probably an extreme logic error here
	for(size_t i = 0; i < index; ++i)
		lines[i] = make_line(lines[i]);


	// imagine doing something optimized lol
	// search for the options
	printf("TOTAL_OPTIONS: %lu\n", TOTAL_OPTIONS);
	for(size_t i = 0; i < TOTAL_OPTIONS; ++i){
		bool found = false;
		for(size_t j = 0; j < index; ++j){
			if(strncmp(options[i].name, lines[j], options[i].str_len) == 0){
				options[i].line = lines[j];
				found = true;
				break;
			}
		}
		if(found)
			printf("option: %s\nline: %s\n", options[i].name, options[i].line);
	}

	// memory leak goes brrrrrr
	return 0;
}
