#include <stdint.h>
#include <stddef.h>

#include "bios_common.h"

void entry_point(struct BiosApi* p_bios)
{
	struct BiosArgs args1 = {
		.tag = Print,
		.print = {
			.ptr = "Hello, world from C!",
			.length = 20
		}
	};
	p_bios->exec(&args1);
	struct BiosArgs args2 = {
		.tag = Exit
	};
	p_bios->exec(&args2);

	// Never get here!
	abort();
}
