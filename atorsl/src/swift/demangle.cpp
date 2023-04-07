#include "swift/Demangling/Demangle.h"

using namespace swift::Demangle;

/// Returns the preferred options to pass to the demangler.
static DemangleOptions DemanglerOptions() {
    auto opts = DemangleOptions();
    opts.SynthesizeSugarOnTypes = true;
    opts.QualifyEntities = true;
    opts.PrintForTypeName = false;

    opts.DisplayModuleNames = false;
    opts.DisplayStdlibModule = false;
    opts.DisplayObjCModule = false;
    opts.DisplayDebuggerGeneratedModule = true;

    opts.DisplayEntityTypes = true;
    opts.DisplayExtensionContexts = true;
    opts.DisplayGenericSpecializations = false;
    opts.DisplayLocalNameContexts = true;
    opts.DisplayProtocolConformances = true;
    opts.DisplayUnmangledSuffix = true;
    opts.DisplayWhereClauses = true;

    opts.ShortenArchetype = false;
    opts.ShortenPartialApply = false;
    opts.ShortenThunk = false;
    opts.ShortenValueWitness = false;

    opts.ShowPrivateDiscriminators = true;
    opts.ShowFunctionArgumentTypes = false;
    opts.ShowAsyncResumePartial = true;
    return opts;
}

/// Demangle the given symbol and return the readable name.
///
/// \param symbol The mangled Swift symbol string.
/// \param buffer A mutable pointer to hold the demangled symbol.
/// \param buffer_length The size of the buffer.
///
/// \return whether the operation was successful (1) or not (0).
extern "C" int demangleSwiftSymbol(
    const char *symbol,
    char *buffer,
    size_t buffer_length
) {
    auto mangled = llvm::StringRef(symbol);
    auto demangled = demangleSymbolAsString(mangled, DemanglerOptions());

    if (demangled.size() == 0 || demangled.size() >= buffer_length) {
        return false;
    }

    memcpy(buffer, demangled.c_str(), demangled.size());
    buffer[demangled.size()] = '\0';
    return true;
}

/// Checks whether the given string is a mangled Swift symbol.
///
/// \param symbol The mangled Swift symbol to check.
///
/// \return whether the string is a mangled Swift symbol (1) or not (0).
extern "C" int isMangledSwiftSymbol(const char *symbol) {
    return isSwiftSymbol(symbol);
}
