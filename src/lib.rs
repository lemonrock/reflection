// This file is part of reflection. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/reflection/master/COPYRIGHT. No part of reflection, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of reflection. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/reflection/master/COPYRIGHT.

#![feature(core_intrinsics)]

pub fn typeOf<T>(_: &T) -> &str
{
	unsafe
	{
		&(std::intrinsics::type_name::<T>())
	}
}

pub trait CanonicalName
{
    fn canonicalName(&self) -> &str
	{
		unsafe
		{
		    // We are allowed to return a reference (&str) here because type_name() returns a 'static str;
		    // if it returned just str, then we'd have a free-before-use problem
			&(*std::intrinsics::type_name::<Self>())
		}
	}
}

impl<T> CanonicalName for T
{
}
