use core::fmt::Display;

use cgp::prelude::*;

cgp_type!( UserId: Async + Display );

cgp_type!( Quantity: Async + Display );

cgp_type!( Currency: Async + Display );

cgp_type!( Password: Async );

cgp_type!( HashedPassword: Async );
