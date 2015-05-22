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

pub struct MsresampCrcf {
     object: ffiliquid::msresamp_crcf,
}

impl MsresampCrcf {

    /// create multi-stage arbitrary resampler
    ///  _r      :   resampling rate [output/input]
    ///  _as     :   stop-band attenuation [dB]
    pub fn new(_r: f32, _as: f32) -> MsresampCrcf {
        let resampler: ffiliquid::msresamp_crcf = unsafe{ffiliquid::msresamp_crcf_create(_r, _as)};
        MsresampCrcf{object: resampler}
    }

    /// get filter delay (output samples)
    pub fn get_delay(&self) -> f32 {
        unsafe{ffiliquid::msresamp_crcf_get_delay(self.object)}
    }

    /// execute multi-stage resampler
    ///  _q      :   msresamp object
    ///  _x      :   input sample array  [size: _nx x 1]
    ///  _nx     :   input sample array size
    ///  _y      :   output sample array [size: variable]
    ///  _ny     :   number of samples written to _y
    pub fn execute(&self, _x: *mut Complex32, _nx: u32, _y: *mut Complex32, _ny: *mut u32) {
        unsafe{ffiliquid::msresamp_crcf_execute(self.object, _x, _nx, _y, _ny)};
    }
}

impl Drop for MsresampCrcf {
    fn drop(&mut self) {
        unsafe{ffiliquid::msresamp_crcf_destroy(self.object)};
    }
}
