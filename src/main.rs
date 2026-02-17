pub mod background_knowledge;
pub mod constraints;
pub mod examples;
pub mod parse;
pub mod problem;
pub mod program;

use crate::problem::Problem;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!\n");
    //settings = Settings();
    let mut problem = Problem::new("examples/kinship-pi/")?;
    println!("{:?}\n", problem);
    let prog = problem.learn_solution(); // (prog, score, stats) = learn_solution(settings);
    if let Some(prog) = prog {
        println!("{:?}\n", prog); // settings.print_prog_score(prog, score);
    } else {
        println!("NO SOLUTION\n");
    }
    Ok(())
    // if settings.show_stats {
    //     stats.show()
    // }
}

// from popper.util import Settings
// from popper.loop import learn_solution

// if __name__ == '__main__':
//     settings = Settings(cmd_line=True)
//     prog, score, stats = learn_solution(settings)
//     if prog != None:
//         settings.print_prog_score(prog, score)
//     else:
//         print('NO SOLUTION')
//     if settings.show_stats:
//         stats.show()
