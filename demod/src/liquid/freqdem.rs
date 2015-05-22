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

pub struct Freqdem {
     object: ffiliquid::freqdem,
}

impl Freqdem {

    /// create freqdem object (frequency demodulator)
    ///  _kf      :   modulation factor
    pub fn new(_kf: f32) -> Freqdem {
        let demod: ffiliquid::freqdem = unsafe{ffiliquid::freqdem_create(_kf)};
        Freqdem{object: demod}
    }

    /// demodulate sample
    ///  _q      :   frequency modulator object
    ///  _r      :   received signal r(t)
    ///  _m      :   output message signal m(t)
    pub fn demodulate(&self, _r: Complex32, _m: *mut f32) {
        unsafe{ffiliquid::freqdem_demodulate(self.object, _r, _m)};
    }

    /// demodulate block of samples
    ///  _q      :   frequency demodulator object
    ///  _r      :   received signal r(t) [size: _n x 1]
    ///  _n      :   number of input, output samples
    ///  _m      :   message signal m(t), [size: _n x 1]
    pub fn demodulate_block(&self, _r: *mut Complex32, _n: u32, _m: *mut f32) {
        unsafe{ffiliquid::freqdem_demodulate_block(self.object, _r, _n, _m)};
    }
}

impl Drop for Freqdem {
    fn drop(&mut self) {
        unsafe{ffiliquid::freqdem_destroy(self.object)};
    }
}
