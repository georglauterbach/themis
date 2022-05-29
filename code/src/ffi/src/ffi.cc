#include <iostream>

extern "C" void call_from_ffi()
{
	std::cout << "This was called through the FFI" << std::endl;
}
