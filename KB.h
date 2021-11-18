//! This file provides a C++ interface to the Rust hfkb module's capi

// C FFI (to Rust)
// Links against capi.rs
struct RustHornFormKb;
extern "C" RustHornFormKb *HornFormKb_new();
extern "C" void HornFormKb_drop(RustHornFormKb *);
extern "C" void HornFormKb_tell(RustHornFormKb *, char const *expr, char const *symbol);
extern "C" unsigned char HornFormKb_ask(RustHornFormKb *, char const *question);

// C++ Wrapper
class HF_KB {
	RustHornFormKb *impl;
public:
	HF_KB(): impl( HornFormKb_new() ) {}
	// This `if` is here as I suspect the move constructor would create a NULL impl HF_KB.
	~HF_KB() { if(impl) HornFormKb_drop(impl); }

	// forbid copying
	HF_KB(HF_KB const &) = delete;
	HF_KB &operator=(HF_KB const &) = delete;

	void TELL(char const *expr, char const *symbol)
	{
		HornFormKb_tell(impl, expr, symbol);
	}
	bool ASK(char const *question)
	{
		return HornFormKb_ask(impl, question);
	}
};
