use crate::{
    opt::{Opt, Options},
};

pub enum OperationMode {
    Version(bool),
    Normal(Options, Vec<String>),
    Help,
}

pub fn parse() -> Result<OperationMode, &'static str>
{
    let mut opt = Options::new();
    let mut files = Vec::new();
    let mut look = true;

    let args: Vec<String> = std::env::args().collect();
    let mut i=1;
    while i < args.len()
    {
	let arg = &args[i];
	match arg.as_str() {
	    "-h" if i == 1 => {
		return Ok(OperationMode::Help);
	    },
	    "-v" if i==1 => {
		return Ok(OperationMode::Version(false));
	    },
	    "-V" if i==1 => {
		return Ok(OperationMode::Version(true));
	    },
	    "-s" if look => {
		opt |= Opt::Silent;
	    },
	    "-e" if look => {
		if i < args.len() - 1 {
		    opt |= Opt::Execute(args[i+1].to_owned());
		} else {
		    return Err("-e expects an argument that wasn't given.");
		}
		i += 1;
	    },
	    "-o" if look  => {
		if i < args.len() - 1 {
		    opt |= Opt::Output(args[i+1].to_owned());
		} else {
		    return Err("-o expects an argument that wasn't given.");
		}
		i += 1;
	    },
	    "-u" if look => {
		opt |= Opt::NoHash;
	    },
	    "-" if look => look=false,
	    other => {
		files.push(other.to_owned());
		look=false;
	    },
	}
	i += 1;
    }

    if files.len() < 1 {
	return Err("No files specified. For help run with `-h`");
    }

    Ok(OperationMode::Normal(opt, files))
}

pub fn program_name() -> String
{
    std::env::args().next().unwrap()
}
