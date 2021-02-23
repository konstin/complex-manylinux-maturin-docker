This repository is an example of how to build manylinux wheels with maturin and github actions if you can't use the konstin2/maturin image.

It contains two crates with pyo3 bindings: nightly-only, which is build which all compatible python versions, and nightly-only-abi3, which is only build with python 3.6. The main trick is to run all commands inside the quay.io/pypa/manylinux2014_x86_64 image, using the `container: quay.io/pypa/manylinux2014_x86_64` directive in the build job.

The `custom-docker-image` folder shows an example how to make a custom image based on konstin2/maturin. In this case we add a more recent cmake version than cent os provides.