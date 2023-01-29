use clap::Parser;
use nucleoguide::GuideDesign;

#[derive(Debug, Parser)]
#[command(version, author, about = &ABOUT, help_template = &HELP)]
struct Args {
    #[arg(index = 1, required = true, help = "Target sequence")]
    seqeunce: String,
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
}

fn main() {
    let args = Args::parse();
    let init = match GuideDesign::new(args.seqeunce, args.pam, args.gsize, args.gxc, args.gic) {
        Ok(n) => n,
        Err(error) => panic!("{}", error),
    };
    match GuideDesign::idgrnas(&init) {
        Ok(n) => println!("{:?}", n),
        Err(error) => panic!("{}", error),
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
Guide RNA design tool built with performance and ease-of-use in mind.
";
