constexpr const int DATA_COUNT = 1;
constexpr const char* const DATA_EXEC_AFTER = nullptr;
static constexpr auto DATA_EXEC_AFTER_HASH = "unbound"_sha256;
constexpr const unsigned char DATA[] = {
	0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x70, 0x6c,
	0x61, 0x6e, 0x65, 0x74, 0x21, 0x0a,
};
constexpr const long DATA_LENGTHS[DATA_COUNT] = {
	14ll,
};
constexpr const char* const DATA_NAMES[DATA_COUNT] = {
	"test.txt",
};
