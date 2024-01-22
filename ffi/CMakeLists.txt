add_library(
	"${PROJECT_NAME}"
	STATIC
	"main.cpp"
	".clang-format"
)

target_compile_features("${PROJECT_NAME}" PRIVATE cxx_std_17)
target_link_libraries("${PROJECT_NAME}" PRIVATE DirectXTex)

if(WIN32)
	target_compile_definitions("${PROJECT_NAME}" PRIVATE CONFIG_WINDOWS=1)
endif()
