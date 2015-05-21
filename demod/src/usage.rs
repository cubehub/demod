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

use clap::{App, Arg, SubCommand};
use self::DataType::{F32, I16};
use self::Modulation::{FM};

use std::fmt;
use std::process::exit;


#[derive(Clone, Copy)]
pub enum DataType {
    F32,
    I16,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataType::F32 => {write!(f, "f32")},
            DataType::I16 => {write!(f, "i16")},
        }
    }
}

pub enum Modulation {
    FM,
    //AM,
}

pub struct FmModulationArgs {
    pub deviation: Option<u32>,
}

pub struct CommandArgs {
    pub samplerate: Option<u32>,
    pub resamplerate: Option<u32>,
    pub inputtype: Option<DataType>,
    pub bandwidth: Option<u32>,
    pub modulation: Option<Modulation>,

    pub fmargs: FmModulationArgs,
}

pub fn args() -> CommandArgs {
    let datatypes = ["i16", "f32"];

    let matches = App::new("demod")
                .author("Andres Vahter <andres.vahter@gmail.com>")
                .version(env!("CARGO_PKG_VERSION"))
                .about("Reads IQ data from stdin, filters and demodulates according to parameters and writes demodulated data back to stdout")

                .arg(Arg::with_name("SAMPLERATE")
                    .long("samplerate")
                    .short("s")
                    .help("IQ data input samplerate")
                    .required(true)
                    .takes_value(true))

                .arg(Arg::with_name("RESAMPLERATE")
                    .long("resamplerate")
                    .short("r")
                    .help("IQ data output samplerate")
                   .required(true)
                   .takes_value(true))

                .arg(Arg::with_name("INTYPE")
                    .long("intype")
                    .short("i")
                    .help("input IQ data type")
                    .required(true)
                    .possible_values(&datatypes)
                    .takes_value(true))

                .arg(Arg::with_name("BANDWIDTH")
                    .long("bandwidth")
                    .help("bandpass filter bandwidth")
                    .required(true)
                    .takes_value(true))

                .subcommand(SubCommand::new("fm")
                    .about("FM demodulation")

                    .arg(Arg::with_name("DEVIATION")
                        .long("deviation")
                        .help("FM deviation [Hz]")
                        .required(true)
                        .takes_value(true)))

                .get_matches();


    let mut args = CommandArgs {
                    samplerate : None,
                    resamplerate : None,
                    inputtype : None,
                    bandwidth : None,
                    modulation : None,

                    fmargs : FmModulationArgs {
                        deviation: None,
                    },
                };

    match matches.subcommand_name() {
        Some("fm") => {
            args.modulation = Some(FM);
            args.samplerate = Some(value_t_or_exit!(matches.value_of("SAMPLERATE"), u32));
            args.resamplerate = Some(value_t_or_exit!(matches.value_of("RESAMPLERATE"), u32));
            args.bandwidth = Some(value_t_or_exit!(matches.value_of("BANDWIDTH"), u32));

            match matches.value_of("INTYPE").unwrap() {
                "f32" => {args.inputtype = Some(F32);},
                "i16" => {args.inputtype = Some(I16);},
                _ => unreachable!()
            }

            let submatches = matches.subcommand_matches("fm").unwrap();
            args.fmargs.deviation = Some(value_t_or_exit!(submatches.value_of("DEVIATION"), u32));
        },

        _ => {
            println!("modulation not specified, check <demod -h> for available modulations");
            exit(1);
        }
    }

    args
}
