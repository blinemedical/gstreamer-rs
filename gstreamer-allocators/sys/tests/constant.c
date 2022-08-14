// Generated by gir (https://github.com/gtk-rs/gir @ 54e116a11822)
// from gir-files (https://github.com/gtk-rs/gir-files @ df20f22974b6)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ e421156aab30)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT(GST_ALLOCATOR_DMABUF);
    PRINT_CONSTANT(GST_ALLOCATOR_FD);
    PRINT_CONSTANT(GST_CAPS_FEATURE_MEMORY_DMABUF);
    PRINT_CONSTANT((guint) GST_FD_MEMORY_FLAG_DONT_CLOSE);
    PRINT_CONSTANT((guint) GST_FD_MEMORY_FLAG_KEEP_MAPPED);
    PRINT_CONSTANT((guint) GST_FD_MEMORY_FLAG_MAP_PRIVATE);
    PRINT_CONSTANT((guint) GST_FD_MEMORY_FLAG_NONE);
    return 0;
}
