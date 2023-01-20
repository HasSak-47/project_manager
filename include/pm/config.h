#ifndef __PM_CONFIG_H__
#define __PM_CONFIG_H__

#ifdef __cpp
extern "C"{
#endif

#include <stdint.h>
#include <stdbool.h>

typedef enum type{
	t_bool,
	t_i32,
	t_f32,
	t_str,
	t_non,
} type;

typedef struct config_data{
	const type t;
	union {
		const bool  b8;
		const int   i32;
		const float f32;
		const char* const str;
	};
} config_data;

const config_data get_config(const char* name);

#ifdef __cpp
}
#endif

#endif
