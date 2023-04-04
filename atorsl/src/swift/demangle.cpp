#include "swift/Demangling/Demangle.h"

using namespace swift::Demangle;

/// Demangle the given symbol and return the readable name.
///
/// \param symbol The mangled Swift  symbol string.
/// \param buffer A mutable pointer
extern "C" int demangleSwiftSymbol(
    const char *symbol,
    char *buffer,
    size_t buffer_length
) {
    DemangleOptions opts;
    opts.SynthesizeSugarOnTypes = true;
    opts.DisplayStdlibModule = false;
    opts.DisplayObjCModule = false;

    std::string demangled = demangleSymbolAsString(llvm::StringRef(symbol), opts);

    if (demangled.size() == 0 || demangled.size() >= buffer_length) {
        return false;
    }

    memcpy(buffer, demangled.c_str(), demangled.size());
    buffer[demangled.size()] = '\0';
    return true;
}

extern "C" int isMangledSwiftSymbol(const char *symbol) {
    return isSwiftSymbol(symbol);
}
