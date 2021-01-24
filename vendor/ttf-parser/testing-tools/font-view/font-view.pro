QT       += widgets

CONFIG += c++14

CONFIG(release, debug|release): LIBS += -L$$PWD/../../c-api/target/release/ -lttfparser
else:CONFIG(debug, debug|release): LIBS += -L$$PWD/../../c-api/target/debug/ -lttfparser

INCLUDEPATH += $$PWD/../../c-api
DEPENDPATH += $$PWD/../../c-api

SOURCES += \
    glyphsview.cpp \
    main.cpp \
    mainwindow.cpp \
    ttfparserfont.cpp

HEADERS += \
    glyph.h \
    glyphsview.h \
    mainwindow.h \
    ttfparserfont.h

FORMS += \
    mainwindow.ui

# qmake DEFINES+=WITH_FREETYPE
contains(DEFINES, WITH_FREETYPE) {
    SOURCES += freetypefont.cpp
    HEADERS += freetypefont.h

    CONFIG += link_pkgconfig
    PKGCONFIG += freetype2
}

# qmake DEFINES+=WITH_HARFBUZZ HARFBUZZ_SRC=/path/to/harfbuzz-master/
contains(DEFINES, WITH_HARFBUZZ) {
    SOURCES += harfbuzzfont.cpp
    HEADERS += harfbuzzfont.h

    # harfbuzz should be built with cmake
    LIBS += -L$$HARFBUZZ_SRC/build -lharfbuzz
    INCLUDEPATH += $$HARFBUZZ_SRC/src
}
