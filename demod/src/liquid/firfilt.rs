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

use liquid::ffiliquid;
use liquid::{Complex32};

#[allow(non_camel_case_types)]
pub enum FirFilterType {
    LIQUID_FIRFILT_UNKNOWN=0,   // unknown filter type

    // Nyquist filter prototypes
    LIQUID_FIRFILT_KAISER,      // Nyquist Kaiser filter
    LIQUID_FIRFILT_PM,          // Parks-McClellan filter
    LIQUID_FIRFILT_RCOS,        // raised-cosine filter
    LIQUID_FIRFILT_FEXP,        // flipped exponential
    LIQUID_FIRFILT_FSECH,       // flipped hyperbolic secant
    LIQUID_FIRFILT_FARCSECH,    // flipped arc-hyperbolic secant

    // root-Nyquist filter prototypes
    LIQUID_FIRFILT_ARKAISER,    // root-Nyquist Kaiser (approximate optimum)
    LIQUID_FIRFILT_RKAISER,     // root-Nyquist Kaiser (true optimum)
    LIQUID_FIRFILT_RRC,         // root raised-cosine
    LIQUID_FIRFILT_hM3,         // harris-Moerder-3 filter
    LIQUID_FIRFILT_GMSKTX,      // GMSK transmit filter
    LIQUID_FIRFILT_GMSKRX,      // GMSK receive filter
    LIQUID_FIRFILT_RFEXP,       // flipped exponential
    LIQUID_FIRFILT_RFSECH,      // flipped hyperbolic secant
    LIQUID_FIRFILT_RFARCSECH,   // flipped arc-hyperbolic secant
}

pub struct FirFilterCrcf {
    object: ffiliquid::firfilt_crcf,
}

impl FirFilterCrcf {

    /// create using Kaiser-Bessel windowed sinc method
    ///  len        : filter length, len > 0
    ///  cutoff     : filter cut-off frequency 0 < cutoff < 0.5
    ///  attenuation: filter stop-band attenuation [dB], attenuation > 0
    ///  offset     : fractional sample offset, -0.5 < offset < 0.5
    pub fn kaiser(len: u32, cutoff: f32, attenuation: f32, offset: f32) -> FirFilterCrcf {
        let filter: ffiliquid::firfilt_crcf = unsafe{ffiliquid::firfilt_crcf_create_kaiser(len, cutoff, attenuation, offset)};
        FirFilterCrcf{object: filter}
    }

    /// create from square-root Nyquist prototype
    ///  _type   : filter type (e.g. LIQUID_FIRFILT_RRC)
    ///  _k      : nominal samples/symbol, _k > 1
    ///  _m      : filter delay [symbols], _m > 0
    ///  _beta   : rolloff factor, 0 < beta <= 1
    ///  _mu     : fractional sample offset,-0.5 < _mu < 0.5
    pub fn rnyquist(_type: FirFilterType, _k: u32, _m: u32, _beta: f32, _mu: f32) -> FirFilterCrcf {
        let filter: ffiliquid::firfilt_crcf = unsafe{ffiliquid::firfilt_crcf_create_rnyquist(_type as i32, _k, _m, _beta, _mu)};
        FirFilterCrcf{object: filter}
    }

    /// set output scaling for filter
    pub fn set_scale(&self, scale: f32) {
        unsafe{ffiliquid::firfilt_crcf_set_scale(self.object, scale)};
    }

    /// push sample into filter object's internal buffer
    ///  _x      : single input sample
    pub fn push(&self, _x: Complex32) {
        unsafe{ffiliquid::firfilt_crcf_push(self.object, _x)}
    }

    /// execute the filter on internal buffer and coefficients
    ///  _y      : pointer to single output sample
    pub fn execute(&self, _y: *mut Complex32) {
        unsafe{ffiliquid::firfilt_crcf_execute(self.object, _y);}
    }

    /// execute the filter on a block of input samples; the
    /// input and output buffers may be the same
    ///  _x      : pointer to input array [size: _n x 1]
    ///  _n      : number of input, output samples
    ///  _y      : pointer to output array [size: _n x 1]
    pub fn execute_block(&self, _x: *mut Complex32, _n: u32, _y: *mut Complex32) {
        unsafe{ffiliquid::firfilt_crcf_execute_block(self.object, _x, _n, _y);}
    }

    /// return length of filter object
    pub fn get_length(&self) -> u32 {
        unsafe{ffiliquid::firfilt_crcf_get_length(self.object)}
    }

    /// compute complex frequency response of filter object
    ///  _fc     : frequency to evaluate
    ///  _h      : pointer to output complex frequency response
    pub fn freqresponse(&self, _fc: f32, _h: *mut Complex32) {
        unsafe{ffiliquid::firfilt_crcf_freqresponse(self.object, _fc, _h);}
    }

    /// compute and return group delay of filter object
    ///  _fc     : frequency to evaluate
    pub fn groupdelay(&self, _fc: f32) -> f32 {
        unsafe{ffiliquid::firfilt_crcf_groupdelay(self.object, _fc)}
    }
}

impl Drop for FirFilterCrcf {
    fn drop(&mut self) {
        unsafe{ffiliquid::firfilt_crcf_destroy(self.object)};
    }
}
