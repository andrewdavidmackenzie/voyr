.PHONY: all
all:

.PHONY: dependencies
dependencies:
	brew install opencv

#export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
#export LDFLAGS=-L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib
#export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib