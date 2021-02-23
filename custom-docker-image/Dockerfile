FROM konstin2/maturin

WORKDIR /
ADD https://github.com/Kitware/CMake/releases/download/v3.19.1/cmake-3.19.1.tar.gz .
RUN tar -xf cmake-3.19.1.tar.gz && \
    cd cmake-3.19.1 && \
    ./bootstrap -- -DCMAKE_USE_OPENSSL=OFF && \
    make -j8 install && \
    cd .. && \
    rm -r cmake*

WORKDIR /io
ENTRYPOINT ["/usr/bin/maturin"]
