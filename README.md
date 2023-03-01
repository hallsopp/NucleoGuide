# nucleoguide

Short Guide RNA (sgRNA) designer written in the Rust programming language. 

## Installation 
- No official release yet, so to install yourself you will have to clone and build locally:
	- Install [rustup](https://www.rust-lang.org/tools/install)
	- `git clone https://github.com/Smilodon-Software/NucleoGuide.git`
	- `cd nucleoguide && cargo build --release` (Note that whilst there is no official release there is also no CI, so it may be a good idea to run `cargo test`)
	- Then run with `./target/release/nucleoguide --help`
- Note that there may potentially be breaking changes merged 

## Features 
- **Currently implemented:**
	- PAM lookup and candidate sgRNA shortlisting/filtering 
		- Could be optimised further using [rayon](https://docs.rs/rayon/latest/rayon/) for potential parallelism, however this could introduce overhead for smaller target sequences, which is likely the common input case. (might be best with a --option)
- **Will be implemented:**
	- Genome wide shortlist alignment using BWT-based semiglobal aligner ([rust-bio](https://docs.rs/bio/latest/bio/index.html)) -- currently working on 
	- Off-target Cutting Frequency Determination (CFD) scores for all matches (Doench et al.)
	- Aggregation of CFD scores into 'likelihood score'

## Contributing 
- Any performance upgrades or desired features are welcome 
- Submit PR as usual, will go through manual review 
