# demod
Command line utility based on liquid-dsp for realtime SDR IQ stream demodulation

## dependencies
### [liquid-dsp](https://github.com/jgaeddert/liquid-dsp)
    git clone git://github.com/jgaeddert/liquid-dsp.git
    cd liquid-dsp
    ./bootstrap.sh
    ./configure
    make
    sudo make install


## install
    git clone https://github.com/cubehub/demod.git
    cd demod
    mkdir build
    cd build
    cmake ../
    make
    sudo make install
