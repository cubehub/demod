/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Andres Vahter (andres.vahter@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// import local modules
extern crate demod;
use demod::usage;
use demod::usage::DataType::{I16, F32};

// import external modules
use std::process::exit;
use std::io;
use std::io::Read;
use std::io::Write;
use std::mem;
use std::slice;

extern crate liquid_dsp;
use liquid_dsp::firfilt;
use liquid_dsp::msresamp;
use liquid_dsp::freqdem;
//use liquid_dsp::{Complex::<f32>::new(};

extern crate num;
use num::complex::Complex;

const BUFFER_SIZE: usize = 8192;

macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

fn main() {
    let args = usage::args();

    println_stderr!("demod {} andres.vahter@gmail.com\n\n", env!("CARGO_PKG_VERSION"));

    // filter options
    let filter_len = 64;
    let filter_cutoff_freq = args.bandwidth.unwrap() as f32 / args.samplerate.unwrap() as f32;
    let filter_attenuation = 70.0f32;

    let filter = firfilt::FirFilterCrcf::kaiser(filter_len, filter_cutoff_freq, filter_attenuation, 0.0f32);
    filter.set_scale(2.0f32 * filter_cutoff_freq);


    // resampler options
    let resampler_rate = if args.resamplerate.is_some() {
                            args.resamplerate.unwrap() as f32 / args.samplerate.unwrap() as f32
                        }
                        else {
                            1.0_f32
                        };

    let resampler = msresamp::MsresampCrcf::new(resampler_rate, filter_attenuation);
	let resampler_delay = resampler.get_delay();

    // TODO, is it so?????
    let num_samples = match args.inputtype.unwrap() {
        I16 => {BUFFER_SIZE as u32 / 2},
        F32 => {BUFFER_SIZE as u32 / 4},
    };

    // number of input samples (zero-padded)
    let resampler_input_len = num_samples + resampler_delay.ceil() as u32 + 10;
	// output buffer with extra padding
	let resampler_output_len = (2f32 * resampler_input_len as f32 * resampler_rate as f32) as u32;

    let mut input = vec![Complex::<f32>::new(0.0f32, 0.0f32); resampler_input_len as usize];
	let mut output = vec![Complex::<f32>::new(0.0f32, 0.0f32); resampler_output_len as usize];
	let mut resampler_output_count = 0;


    // FM demodulator
    let modulation_factor = if args.resamplerate.is_some() {
                                args.fmargs.deviation.unwrap() as f32 / args.resamplerate.unwrap() as f32
                            }
                            else {
                                args.fmargs.deviation.unwrap() as f32 / args.samplerate.unwrap() as f32
                            };

    let fm_demod = freqdem::Freqdem::new(modulation_factor);
    let mut demod_f32_out = vec![0_f32; resampler_output_len as usize];
    let mut demod_i16_out = vec![0_i16; resampler_output_len as usize];


    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut inbuf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    loop {
        match stdin.read(&mut inbuf) {
            Ok(size) => {
                let mut sample_count: usize = 0;

                match args.inputtype.unwrap() {
                    I16 => {
                        for b in inbuf.chunks(4) {
                            let i: f32 = ((b[1] as i16) << 8 | b[0] as i16) as f32 / 32768.;
                            let q: f32 = ((b[3] as i16) << 8 | b[2] as i16) as f32 / 32768.;

                            input[sample_count] = Complex::<f32>::new(i, q);
                            filter.push(input[sample_count]);
                            filter.execute(&mut input[sample_count]);
                            sample_count += 1;
                        }
                    }
                    F32 => {
                        for b in inbuf.chunks(8) {
                            let i: f32 = unsafe {mem::transmute::<u32, f32>(((b[3] as u32) << 24) | ((b[2] as u32) << 16) | ((b[1] as u32) << 8) | b[0] as u32)};
                            let q: f32 = unsafe {mem::transmute::<u32, f32>(((b[7] as u32) << 24) | ((b[6] as u32) << 16) | ((b[5] as u32) << 8) | b[4] as u32)};

                            input[sample_count] = Complex::<f32>::new(i, q);
                            filter.push(input[sample_count]);
                            filter.execute(&mut input[sample_count]);
                            sample_count += 1;
                        }
                    }
                }

                //let slice = unsafe {slice::from_raw_parts(input.as_ptr() as *const _, size * 2)};
                //stdout.write(&slice).unwrap();

                // resample
                resampler.execute(&mut input, sample_count as u32, &mut output, &mut resampler_output_count);

                //let slice = unsafe {slice::from_raw_parts(output.as_ptr() as *const _, (resampler_output_count * 8) as usize)};
                //stdout.write(&slice).unwrap();

                // demodulate
                fm_demod.demodulate_block(&mut output, resampler_output_count, &mut demod_f32_out);


                match args.outputtype.unwrap() {
                    I16 => {
                        for i in 0 .. resampler_output_count as usize {
                            if args.fmargs.squarewave.unwrap() {
                                // make output square like, multimon-ng likes it more
                                if demod_f32_out[i] > 0.0 {
                                    demod_f32_out[i] = 1.0;
                                }
                                if demod_f32_out[i] < 0.0 {
                                    demod_f32_out[i] = -1.0;
                                }

                            }
                            else {
                                // clamp output
                                if demod_f32_out[i] > 1.0 {
                                    demod_f32_out[i] = 1.0;
                                }
                                if demod_f32_out[i] < -1.0 {
                                    demod_f32_out[i] = -1.0;
                                }
                            }

                            demod_i16_out[i] = (demod_f32_out[i] * 32767_f32) as i16;
                        }

                        let slice = unsafe {slice::from_raw_parts(demod_i16_out.as_ptr() as *const _, (resampler_output_count * 2) as usize)};
                        stdout.write(&slice).map_err(|e|{println_stderr!("demod stdout.write error: {}", e); exit(1);});
                        stdout.flush().map_err(|e|{println_stderr!("demod stdout.flush error: {}", e); exit(1);});
                    }
                    F32 => {
                        let slice = unsafe {slice::from_raw_parts(demod_f32_out.as_ptr() as *const _, (resampler_output_count * 4) as usize)};
                        stdout.write(&slice).map_err(|e|{println_stderr!("stdout.write error: {}", e); exit(1);});
                        stdout.flush().map_err(|e|{println_stderr!("stdout.flush error: {}", e); exit(1);});
                    }
                }

                if size < BUFFER_SIZE {
                    break;
                }
            }
            Err(e) => {
                println_stderr!("err: {:?}", e);
                break;
            }
        }
    }
}
