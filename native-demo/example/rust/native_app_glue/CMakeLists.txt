cmake_minimum_required(VERSION 3.6)

set(NATIVE_APP_GLUE_PATH ${CMAKE_SOURCE_DIR})

add_library(native_app_glue
  STATIC
    ${NATIVE_APP_GLUE_PATH}/android_native_app_glue.c
    ${NATIVE_APP_GLUE_PATH}/android_native_app_glue.h
    ${NATIVE_APP_GLUE_PATH}/main.c
    )

target_link_libraries(native_app_glue
    android
    EGL
    GLESv1_CM
    log
    )

# # Export ANativeActivity_onCreate(),
# # Refer to: https://github.com/android-ndk/ndk/issues/381.
# set(CMAKE_SHARED_LINKER_FLAGS
#         "${CMAKE_SHARED_LINKER_FLAGS} -u ANativeActivity_onCreate")

install (TARGETS native_app_glue DESTINATION .)
