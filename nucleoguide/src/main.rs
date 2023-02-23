use clap::Parser;
use nucleoguide::GuideDesign;

#[derive(Debug, Parser)]
#[command(version, author, about = &ABOUT, help_template = &HELP)]
struct Args {
    #[arg(index = 1, required = true, help = "Target sequence")]
    sequence: String,
    #[arg(short, long, default_value = "NGG", help = "PAM sequence")]
    pam: String,
    #[arg(
        id = "grna-size",
        short,
        long,
        help = "size of gRNAs to return",
        default_value = "20"
    )]
    gsize: usize,
    #[arg(
        id = "grna-exclusion-pattern",
        long,
        help = "Remove gRNAs if they contain this",
        default_value = ""
    )]
    gxc: String,
    #[arg(
        id = "grna-inclusion-pattern",
        long,
        help = "Only return gRNAs if they contain this",
        default_value = ""
    )]
    gic: String,
    #[arg(
        id = "grna-gc-min",
        long,
        help = "Min %GC content for guides",
        default_value = "40"
    )]
    ggcmin: f32,
    #[arg(
        id = "grna-gc-max",
        long,
        help = "Max %GC content for guides",
        default_value = "70"
    )]
    ggcmax: f32,
}

fn main() {
    let args = Args::parse();
    let init = match GuideDesign::new(
        args.sequence,
        args.pam,
        args.gsize,
        args.gxc,
        args.gic,
        args.ggcmin,
        args.ggcmax,
    ) {
        Ok(n) => n,
        Err(error) => panic!("{}", error),
    };
    match init.idgrnas() {
        Ok(n) => println!("{n:?}"),
        Err(error) => panic!("{error}"),
    }
}

const HELP: &str = "\
\n
{before-help}
{name} {version}
{author-with-newline} 
{about-with-newline}
{usage-heading} {usage}

{all-args}
{after-help}
";

const ABOUT: &str = "\
Design your guides with NucleoGuide!
";
