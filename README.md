# demod
Command line utility based on liquid-dsp for demodulating SDR IQ streams.
Firstly it was written in C ([last commit to C version](https://github.com/cubehub/demod/commit/1b1736ec72adc5b36db951be41dceaf3badccea9)), however now it is rewritten in [rust](http://www.rust-lang.org).

## dependencies
### [liquid-dsp](https://github.com/jgaeddert/liquid-dsp)
    git clone git://github.com/jgaeddert/liquid-dsp.git
    cd liquid-dsp
    ./bootstrap.sh
    ./configure
    make
    sudo make install


### rust
http://www.rust-lang.org/install.html

    curl -sSf https://static.rust-lang.org/rustup.sh | sh

## build

    git clone https://github.com/cubehub/demod.git
    cd demod
    cargo build --release

## install
### mac os x

    cp target/release/demod /usr/local/bin/

### linux

    sudo cp target/release/demod /usr/local/bin/

## usage
play FM radio recording (deemph filter not used and does not play in stereo)

    cat fm_radio_i16_rec.iq | demod --samplerate 230400 --intype i16 --outtype i16 --bandwidth 100000 fm --deviation 75000 | play -t raw -r 230.4k -e signed-integer -b16 -c 1 -V1 -
    cat fm_radio_f32_rec.iq | demod --samplerate 230400 --intype f32 --outtype f32 --bandwidth 100000 fm --deviation 75000 | play -t raw -r 230.4k -e floating-point -b32 -c 1 -V1 -

demodulate FSK9600 raw IQ data recording and pipe output to multimon-ng for packet decoding, notice `--squarewave` flag is added to FM demodulation, which makes demodulator output square like (multimon-ng likes it more)

    sox -t wav sdr_fsk9600.wav -esigned-integer -b16  -r 126000 -t raw - | demod --samplerate 126000 --resamplerate 48000 --bandwidth 4500 fm --deviation 3500 --squarewave | multimon-ng -t raw -a FSK9600 /dev/stdin

for testing AX25 decoding use this [ax25_fsk9600_1024k_i16.wav](https://github.com/cubehub/samples/blob/master/ax25_fsk9600_1024k_i16.wav) with the following command (install `doppler` from [here](https://github.com/cubehub/doppler)):

    sox -t wav ax25_fsk9600_1024k_i16.wav -esigned-integer -b16  -r 126000 -t raw - | doppler const -s 126000 -i i16 --shift 14500 | demod -s 126000 -r 48000 -i i16 -o i16 --bandwidth 4500 fm --deviation 3500 --squarewave | multimon-ng -t raw -a FSK9600 /dev/stdin

Notice that here [modified multimon-ng](https://github.com/cubehub/multimon-ng) is used that supports 48000 sps input stream for fsk9600 decoder. Read [here](http://andres.svbtle.com/pipe-sdr-iq-data-through-fm-demodulator-for-fsk9600-ax25-reception) why multimon-ng must be modified instead of converting **demod** output to native 22050 format.
