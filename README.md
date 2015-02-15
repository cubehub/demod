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

## usage
demodulate FSK9600 raw IQ data recording and pipe output to multimon-ng for packet decoding

    sox -t wav sdr_fsk9600.wav -esigned-integer -b16  -r 126000 -t raw - | demod -s 126000 -r 48000 -b 4500 -m FM d=3500 | multimon-ng -t raw  -a FSK9600 /dev/stdin

Notice that here [modified multimon-ng](https://github.com/cubehub/multimon-ng) is used that supports 48000 sps input stream for fsk9600 decoder. Read [here](http://andres.svbtle.com/pipe-sdr-iq-data-through-fm-demodulator-for-fsk9600-ax25-reception) why multimon-ng must be modified instead of converting **demod** output to native 22050 format.
