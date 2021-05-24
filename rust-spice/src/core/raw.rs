/*!
A Rust idiomatic CSPICE wrapper built with [procedural macros][`spice_derive`].
*/

use crate::c;
use spice_derive::{cspice_proc, return_output};

cspice_proc!(
    /**
    Close a DAS file.
    */
    pub fn dascls(handle: i32) {}
);

cspice_proc!(
    /**
    Open a DAS file for reading.
    */
    pub fn dasopr<S: Into<String>>(fname: S) -> i32 {}
);

cspice_proc!(
    /**
    Begin a forward segment search in a DLA file.
    */
    pub fn dlabfs(handle: i32) -> (c::SpiceDLADescr, bool) {}
);

cspice_proc!(
    /**
    Return the DSK descriptor from a DSK segment identified  by a DAS handle and DLA descriptor.
    */
    pub fn dskgd(handle: i32, dladsc: c::SpiceDLADescr) -> c::SpiceDSKDescr {}
);

cspice_proc!(
    /**
    Compute the unit normal vector for a specified plate from a type 2 DSK segment.
    */
    pub fn dskn02(handle: i32, dladsc: c::SpiceDLADescr, plid: i32) -> [f64; 3] {}
);

cspice_proc!(
    /**
    Determine the plate ID and body-fixed coordinates of the intersection of a specified ray with
    the surface defined by a type 2 DSK plate model.
    */
    pub fn dskx02(
        handle: i32,
        dladsc: c::SpiceDLADescr,
        vertex: [f64; 3],
        raydir: [f64; 3],
    ) -> (i32, [f64; 3], bool) {
    }
);

cspice_proc!(
    /**
    Load one or more SPICE kernels into a program.
    */
    pub fn furnsh<S: Into<String>>(name: S) {}
);

cspice_proc!(
    /**
    Convert from latitudinal coordinates to rectangular coordinates.
    */
    pub fn latrec(radius: f64, longitude: f64, latitude: f64) -> [f64; 3] {}
);

cspice_proc!(
    /**
    Determines the occultation condition (not occulted, partially, etc.) of one target relative to
    another target as seen by an observer at a given time, with targets modeled as points,
    ellipsoids, or digital shapes (DSK)
    */
    pub fn occult<S: Into<String>>(
        targ1: S,
        shape1: S,
        frame1: S,
        targ2: S,
        shape2: S,
        frame2: S,
        abcorr: S,
        obsrvr: S,
        et: f64,
    ) -> i32 {
    }
);

cspice_proc!(
    /**
    Return the matrix that transforms position vectors from one specified frame to another at a
    specified epoch.
    */
    pub fn pxform<S: Into<String>>(from: S, to: S, et: f64) -> [[f64; 3]; 3] {}
);

cspice_proc!(
    /**
    Return the 3x3 matrix that transforms position vectors from one specified frame at a specified
    epoch to another specified frame at another specified epoch.
    */
    pub fn pxfrm2<S: Into<String>>(from: S, to: S, etfrom: f64, etto: f64) -> [[f64; 3]; 3] {}
);

cspice_proc!(
    /**
    Convert rectangular coordinates to range, right ascension, and declination.
    */
    pub fn recrad(rectan: [f64; 3]) -> (f64, f64, f64) {}
);

cspice_proc!(
    /**
    Return the position of a target body relative to an observing body, optionally corrected for
    light time (planetary aberration) and stellar aberration.
    */
    pub fn spkpos<S: Into<String>>(
        targ: S,
        et: f64,
        frame: S,
        abcorr: S,
        obs: S,
    ) -> ([f64; 3], f64) {
    }
);

cspice_proc!(
    /**
    Convert a string representing an epoch to a double precision value representing the number of
    TDB seconds past the J2000 epoch corresponding to the input epoch.
    */
    pub fn str2et<S: Into<String>>(targ: S) -> f64 {}
);

cspice_proc!(
    /**
    This routine converts an input epoch represented in TDB seconds past the TDB epoch of J2000 to a
    character string formatted to the specifications of a user's format picture.
    This function has a [neat version][crate::neat::timout].
    */
    pub fn timout<S: Into<String>>(et: f64, pictur: S, lenout: usize) -> String {}
);

cspice_proc!(
    /**
    Unload a SPICE kernel.
    */
    pub fn unload<S: Into<String>>(name: S) {}
);

cspice_proc!(
    /**
    Find the separation angle in radians between two double precision, 3-dimensional vectors. This
    angle is defined as zero if either vector is zero.
    */
    #[return_output]
    pub fn vsep(v1: [f64; 3], v2: [f64; 3]) -> f64 {}
);
