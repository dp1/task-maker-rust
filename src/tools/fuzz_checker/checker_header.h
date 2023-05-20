#include <stdlib.h>

// Override the common exit functions, EXIT will throw an exception caught by
// the fuzzer
#define main(...) MAIN(int argc, char **argv)
#define _exit EXIT
#define exit EXIT

[[noreturn]] void EXIT(int status);
