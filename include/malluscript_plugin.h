#ifndef MALLUSCRIPT_PLUGIN_H
#define MALLUSCRIPT_PLUGIN_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    MS_TYPE_STRING = 0,
    MS_TYPE_INTEGER = 1,
    MS_TYPE_BOOL = 2,
    MS_TYPE_FLOAT = 3,
    MS_TYPE_LIST = 4,
    MS_TYPE_REF = 5,
    MS_TYPE_UNKNOWN = 6,
    MS_TYPE_ERROR = 7,
} MS_Type;

typedef struct {
    MS_Type value_type;
    void* data;
} MS_Value;

typedef struct {
    size_t length;
    MS_Value** items;
} MS_List;

typedef struct {
    char* message;
} MS_Error;

typedef MS_Value* (*MS_Native)(const MS_Value* const* args, size_t argc, MS_Error** error);

typedef void (*MS_AddFunction)(void* executor, const char* name, MS_Native func);
typedef MS_Value* (*MS_AllocateValue)(MS_Type v_type);
typedef void (*MS_FreeValue)(MS_Value* ptr);
typedef char* (*MS_AllocateString)(const char* s);
typedef void (*MS_FreeString)(char* ptr);
typedef MS_List* (*MS_AllocateList)(size_t length);
typedef void (*MS_FreeList)(MS_List* ptr);

typedef struct {
    void* executor;
    MS_AddFunction add_function;
    MS_AllocateValue allocate_value;
    MS_FreeValue free_value;
    MS_AllocateString allocate_string;
    MS_FreeString free_string;
    MS_AllocateList allocate_list;
    MS_FreeList free_list;
} MS_InterpreterState;

#ifdef __cplusplus
}
#endif

#endif
