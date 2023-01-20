#include <pm/config.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/* config:
 * path
 */

typedef struct imut_str{
	const char* const name;
	
}imut_str;

typedef struct config_option{
	const char* const  name;
	const size_t len;
	config_data data;
} config_option;


#define MAKE_CONFIG(name, type) (config_option){name, sizeof(name) -1, type}

config_option options[] = {
	MAKE_CONFIG("path", t_str),
};

#define TOTAL_CONFIG (sizeof(options) / sizeof(config_options))

char* make_line(const char* str){
	const char* str_c = str;
	while(*str_c != '\n')
		str_c++;

	size_t len = str_c - str;
	char* line = calloc(len, sizeof(char));

	strncpy(line, str, len);

	return line;
}

void load_options(const char* path){
	FILE* f = fopen(path, "r");

	if(f == NULL){
		printf("could not find config file");
		goto CRASH;
	}

	fseek(f, 0, SEEK_END);
	size_t file_size = ftell(f);
	fseek(f, 0, SEEK_SET);

	//the plus one is to make it null terminated
	char* file_buffer = calloc(file_size + 1, sizeof(char));
	fread(file_buffer, 1, file_size, f);

	if(fclose(f)){
		printf("failed to close file");
		goto CRASH;
	}

	size_t line_count = 0;
	for(size_t i = 0; i < file_size; ++i){
		if(file_buffer[i] == '\n')
			line_count++;
	}

	char** line_buffer = calloc(line_count, sizeof(char*));
	size_t line_index = 0;
	char prev = '\n';
	for(size_t i = 0; i < line_count; ++i){
		if(prev == '\n' && file_buffer[i] != '\n')
			line_buffer[line_index] = &file_buffer[i];
		prev = file_buffer[i];
	}

	for(size_t i = 0; i < line_count; ++i)
		line_buffer[i] = make_line(line_buffer[i]);


	return;
CRASH:
	exit(EXIT_FAILURE);
}

